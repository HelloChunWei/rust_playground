// 選擇排序法
// 重點：
// 1. 簡單直觀：容易理解和實現
// 2. 不斷選擇最小元素：每次從未排序部分選出最小元素
// 3. 交換位置：將選出的最小元素與未排序部分的第一個元素交換
// 4. 時間複雜度：O(n^2)，不受輸入數據的影響
// 5. 原地排序：不需要額外的存儲空間
// 6. 不穩定排序：相等元素的相對位置可能改變

// 結論：
// - 優點：實現簡單，對小規模數據效果好
// - 缺點：對大規模數據效率低，不適合已經接近排序的數據
// - 特色：時間複雜度恆定，不受輸入數據影響
// - 適用場景：小規模數據排序，或作為其他算法的子程序
pub fn select_sort(arr: [u8; 7]) -> [u8; 7] {
    let mut arr = arr;
    for i in 0..arr.len() {
        for j in i..arr.len() {
            let min = arr[i];
            if min > arr[j] {
                arr.swap(i, j);
            }
        }
    }
    arr
}

// 插入排序法
// 插入排序法是一種簡單直觀的排序法，它的基本作法是將一個數列分成已排序與未排序兩部分
// 指標左邊的，一定是排序過的，指標右邊的，一定是未排序的
// 所以當 arr[j] > arr[j-1] 時，代表 arr[j] 絕對比 arr[j-1] 到 arr[0] 的大，那就代表後面的就不需要比對，直接 i++
// 時間複雜度：O(n^2)
// - 時間複雜度：最壞情況 O(n^2)，最好情況 O(n)（當數據已經有序時），跟選擇排序不一樣，選擇排序不管怎樣他就是 O(n^2)
// - 空間複雜度：O(1)，原地排序算法
// - 穩定性：穩定排序
// - 優點：實現簡單，對於小規模或接近有序的數據效率高
// - 缺點：對於大規模亂序數據效率較低
// - 適用場景：小規模數據，接近有序的數據，或作為其他複雜排序算法的子過程
pub fn insert_sort(arr: [u8; 7]) -> [u8; 7] {
    let mut arr = arr;
    for i in 1..arr.len() {
        println!("i: {}", i);
        for j in (1..=i).rev() {
            println!("j: {}", j);
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                println!("arr {:?}", arr);
            } else {
                break;
            }
        }
    }
    arr
}

// 合併排序
// 從中間劃一刀，分左右兩邊，左右兩邊也是一樣從中間畫一刀，直到左右大小為一，這個時候開始 merge
// 重點：
// 1. 分治法：將大問題分解為小問題，逐步解決
// 2. 遞迴：不斷將數組分割，直到只剩一個元素
// 3. 合併：將已排序的子數組合併成更大的有序數組
// 4. 穩定性：保持相等元素的原始順序
// 5. 空間複雜度：需要額外的空間來存儲合併結果

// 結論：
// - 時間複雜度：O(n log n)，無論最好、最壞或平均情況
// - 空間複雜度：O(n)，需要額外的臨時數組
// - 優點：效率高，穩定，適合大數據量
// - 缺點：需要額外空間，對於小數組可能不如插入排序
// - 適用場景：大型數據集，外部排序，需要穩定排序時
pub fn merge_sort(arr: [u8; 7]) -> [u8; 7] {
    fn merge(left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let (mut i, mut j) = (0, 0);

        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                result.push(left[i]);
                i += 1;
            } else {
                result.push(right[j]);
                j += 1;
            }
        }
        // 添加剩餘的，example: 1,5 and 2,4
        // 1會先放入，2 跟五比較 2 放入 -> 1,2
        // 5 跟4比較 4放入 -> 1,2,4
        // 但因為是   while i < left.len() && j < right.len() 所以 while 會中斷，要把剩下的五放入
        // result -> 1, 2, 4, 5
        result.extend_from_slice(&left[i..]);
        result.extend_from_slice(&right[j..]);

        result
    }
    fn sort(list: &[u8]) -> Vec<u8> {
        if list.len() <= 1 {
            return list.to_vec();
        }
        let mid = list.len() / 2;
        let left = sort(&list[..mid]);
        let right = sort(&list[mid..]);

        merge(&left, &right)
    }
    let sorted = sort(&arr);
    sorted.try_into().expect("incorrect length")
}
// 原始版本的 quick sort -> 空間複雜度超級高，一大堆Vec
pub fn quick_sort(arr: [u8; 7]) -> [u8; 7] {
    fn sort(list: &[u8]) -> Vec<u8> {
        if list.len() <= 1 {
            return list.to_vec();
        }
        let pivot = list[list.len() - 1];
        let mut left: Vec<u8> = Vec::new();
        let mut right: Vec<u8> = Vec::new();
        let mut equal: Vec<u8> = Vec::new();

        for &element in list.iter() {
            if element < pivot {
                left.push(element);
            } else if element > pivot {
                right.push(element);
            } else {
                equal.push(element);
            }
        }

        let mut result = sort(&left);
        result.extend_from_slice(&equal);
        result.extend(sort(&right));
        result
    }
    sort(&arr).try_into().expect("incorrect length")
}

//  Lomuto partition scheme
pub fn quick_sort_optimization(arr: [u8; 7]) -> [u8; 7] {
    // partition 是回傳整理過後的 pivot ，pivot 的左邊一定比 pivot 小 or 等於
    // 右邊一定是比較大。
    // 接著在 sort 的 if pi > 0 判斷 p是不是在 0 ，不是的話代表左邊必須要排序
    fn partition(arr: &mut [u8], low: usize, high: usize) -> usize {
        let pivot = arr[high];
        let mut i = low;
        for j in low..high {
            if arr[j] <= pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, high);
        i
    }
    // 因為是針對原本的list 排序，所以sort是不會return 資料的
    fn sort(arr: &mut [u8], low: usize, high: usize) {
        println!("low: {}", low);
        println!("high: {}", high);
        if low < high {
            println!("original arr: {:?}", arr);
            let pi = partition(arr, low, high);
            println!("pi: {}", pi);
            println!("arr: {:?}", arr);
            println!("----");
            if pi > 0 {
                sort(arr, low, pi - 1);
            }
            sort(arr, pi + 1, high);
        }
    }

    let mut arr = arr;
    let len = arr.len();
    if len > 1 {
        sort(&mut arr, 0, len - 1);
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_sort1() {
        let test1: [u8; 7] = [7, 10, 5, 30, 20, 15, 40];
        assert_eq!([5, 7, 10, 15, 20, 30, 40], select_sort(test1));
    }

    #[test]
    fn select_sort2() {
        let test2: [u8; 7] = [80, 1, 5, 2, 2, 32, 5];
        assert_eq!([1, 2, 2, 5, 5, 32, 80], select_sort(test2));
    }

    #[test]
    fn insert_sort1() {
        let test2: [u8; 7] = [80, 1, 5, 2, 2, 32, 5];
        assert_eq!([1, 2, 2, 5, 5, 32, 80], insert_sort(test2));
    }

    #[test]
    fn insert_sort2() {
        let test4: [u8; 7] = [5, 8, 23, 8, 99, 1, 12];
        assert_eq!([1, 5, 8, 8, 12, 23, 99], insert_sort(test4));
    }

    #[test]
    fn merge_sort1() {
        let test5: [u8; 7] = [5, 8, 23, 8, 99, 1, 12];
        assert_eq!([1, 5, 8, 8, 12, 23, 99], merge_sort(test5));
    }

    #[test]
    fn quick_sort1() {
        let test5: [u8; 7] = [5, 8, 23, 8, 99, 1, 12];
        assert_eq!([1, 5, 8, 8, 12, 23, 99], quick_sort(test5));
    }
    #[test]
    fn quick_sort_optimization1() {
        let test5: [u8; 7] = [5, 8, 23, 8, 99, 1, 12];
        assert_eq!([1, 5, 8, 8, 12, 23, 99], quick_sort_optimization(test5));
    }
}
