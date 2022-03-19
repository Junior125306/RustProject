#[cfg(test)]
mod tests {

    // 测试自定义类型
    #[test]
    fn json_to_tabl_test() {
        let test_json = r#"{
        "t_userinfo": {
            "key": "f_userinfo_id",
            "fields": {
                "f_userinfo_id": {
                    "Int": {
                        "length": 25,
                        "minus": false
                    }
                },
                "f_user_state": "String",
                "address": {
                    "OneToOne": ["t_user_address", "f_userinfo_id"]
                },
                "sellinggas": {
                    "OneToMany": ["t_sellinggas", "f_userinfo_id"]
                },
                "handplan": {
                    "OneToMany": ["t_handplan", "f_userinfo_id"]
                }
            }
        }
    }"#;
        let tables: Tables = serde_json::from_value(jtest_jsonson.clone())?;
        println!("============{:?}", tables)
    }

    #[test]
    fn iterator_demo() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        // iter 在不可变引用上创建迭代器
        // into_iter 创建迭代器会获得所有权
        // iter_mut 迭代可变的引用
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }

    #[test]
    fn iterator_aaaaa() {
        let iss: u16 = 65535;
        println!("========={}", iss)
    }

    #[test]
    fn iter_sum() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|x| x.size == shoe_size).collect()
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 13,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_my_size(shoes, 13);
        println!("{:?}", in_my_size)
    }
}
