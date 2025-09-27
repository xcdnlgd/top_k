package main

import (
	"bufio"
	"container/heap"
	"encoding/binary"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"sync"
	"unsafe"
)

type MinHeap []float64

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i] < h[j] } // 小根堆
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *MinHeap) Push(x any) {
	*h = append(*h, x.(float64))
}

func (h *MinHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

// 获取区间 [start,end) 的 top-k
func getTopK(k int, filePath string, start, end int64) *MinHeap {
	f, err := os.Open(filePath)
	if err != nil {
		log.Fatalf("cannot open %s: %v", filePath, err)
	}
	defer f.Close()

	_, err = f.Seek(start, 0)
	if err != nil {
		log.Fatalf("seek failed: %v", err)
	}
	reader := bufio.NewReader(f)

	h := &MinHeap{}
	heap.Init(h)

	buf := make([]byte, 8)
	count := (end - start) / 8
	for range count {
		_, err := reader.Read(buf)
		if err != nil {
			break
		}
		num := binary.LittleEndian.Uint64(buf)
		val := float64FromBits(num)

		if h.Len() < k {
			heap.Push(h, val)
		} else if val > (*h)[0] {
			heap.Pop(h)
			heap.Push(h, val)
		}
	}
	return h
}

// float64FromBits 的封装
func float64FromBits(bits uint64) float64 {
	return *(*float64)(unsafe.Pointer(&bits))
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Please enter the file name")
		return
	}
	filePath := os.Args[1]
	k := 100
	numThreads := 16

	fi, err := os.Stat(filePath)
	if err != nil {
		log.Fatalf("stat failed: %v", err)
	}
	fileSize := fi.Size()
	chunkSize := fileSize / int64(numThreads)

	var wg sync.WaitGroup
	results := make(chan *MinHeap, numThreads)

	// 启动多个 goroutine
	for i := range numThreads {
		start := int64(i) * chunkSize
		end := start + chunkSize
		if i == numThreads-1 {
			end = fileSize
		}
		wg.Add(1)
		go func(start, end int64, id int) {
			defer wg.Done()
			fmt.Printf("thread %d start\n", id)
			h := getTopK(k, filePath, start, end)
			fmt.Printf("thread %d finished\n", id)
			results <- h
		}(start, end, i)
	}

	go func() {
		wg.Wait()
		close(results)
	}()

	// 合并结果
	finalHeap := <-results
	for h := range results {
		for _, num := range *h {
			if num > (*finalHeap)[0] {
				heap.Pop(finalHeap)
				heap.Push(finalHeap, num)
			}
		}
	}

	// 保存结果
	saveResult(finalHeap, "result")
}

func saveResult(h *MinHeap, filePath string) {
	f, err := os.Create(filePath)
	if err != nil {
		log.Fatalf("cannot create result file: %v", err)
	}
	defer f.Close()

	// 排序输出
	result := make([]float64, len(*h))
	copy(result, *h)
	// 从大到小排序
	sort.Slice(result, func(i, j int) bool { return result[i] > result[j] })

	buf := make([]byte, 8)
	for _, num := range result {
		binary.LittleEndian.PutUint64(buf, math.Float64bits(num))
		_, err := f.Write(buf)
		if err != nil {
			log.Fatalf("write failed: %v", err)
		}
	}
	f.Sync()
}
