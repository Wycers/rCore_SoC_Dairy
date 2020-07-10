# rCore_SoC_Dairy

# 2020-7-9

## 回顾

- 刷 Rustlings

## Lab

……

## Rustlings

关于所有权转移的`move_semantics2`这题挺好的，有三种做法：

1. 
    ``` rust

    fn main() {
        let vec0 = Vec::new();

        let mut vec1 = fill_vec(vec0.clone());

        // Do not change the following line!
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

        vec1.push(88);

        println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    }

    fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
        let mut vec = vec;

        vec.push(22);
        vec.push(44);
        vec.push(66);

        vec
    }
    ```
2.
    ``` rust
    fn main() {
        let vec0 = Vec::new();

        let mut vec1 = fill_vec(&vec0);

        // Do not change the following line!
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

        vec1.push(88);

        println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    }

    fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
        let mut vec = vec.clone();

        vec.push(22);
        vec.push(44);
        vec.push(66);

        vec
    }

    ```
3. 
    ``` rust
    
    fn main() {
        let mut vec0 = Vec::new();

        fill_vec(&mut vec0);

        // Do not change the following line!
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    }

    fn fill_vec(vec: &mut Vec<i32>) {
        vec.push(22);
        vec.push(44);
        vec.push(66);   
    }

    ```