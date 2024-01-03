// 選擇排序法
// 選擇排序法是一種簡單直觀的排序法，它的基本作法是從未排序的數列中找出最小的元素
// 將它與未排序數列的第一個元素交換位置，接著再從剩下的未排序數列中找出最小的元素
// 將它與未排序數列的第一個元素交換位置，以此類推，直到所有元素都排序完畢為止。
// 時間複雜度：O(n^2)
// 時間複雜度跟資料的順序無關，不管是什麼順序都是 O(n^2)
// 這是他的特色
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
// 但是如果資料是有序的，那就會是 O(n)，跟選擇排序不一樣，選擇排序不管怎樣他就是 O(n^2)
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
}
