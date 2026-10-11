use namedivider_rs::divider::gbdt_name_divider::get_gbdt_name_divider;
use namedivider_rs::divider::name_divider::NameDivider;
use std::sync::Arc;
use std::thread;

/// テスト用の日本人名前のサンプル
const TEST_NAMES: &[&str] = &[
    "菅義偉",
    "田中太郎",
    "佐藤花子",
    "山田一郎",
    "鈴木二郎",
    "高橋三郎",
    "渡辺四郎",
    "伊藤五郎",
    "中村六郎",
    "小林七郎",
];

#[test]
fn test_gbdt_name_divider_single_thread() {
    // 単一スレッドでの正常動作確認
    let divider = get_gbdt_name_divider(" ".to_string(), true, "gbdt".to_string());
    
    for name in TEST_NAMES {
        let divided_name = divider.divide_name(&name.to_string());
        assert!(!divided_name.family.is_empty());
        assert!(!divided_name.given.is_empty());
        println!("✓ {}: {} | {}", name, divided_name.family, divided_name.given);
    }
}

#[test]
fn test_gbdt_name_divider_multi_thread_concurrent_access() {
    let divider = Arc::new(get_gbdt_name_divider(" ".to_string(), true, "gbdt".to_string()));
    let mut handles = vec![];

    for thread_id in 0..10 {
        let divider_clone = Arc::clone(&divider);
        let handle = thread::spawn(move || {
            for i in 0..50 {
                let name_index = (thread_id * 50 + i) % TEST_NAMES.len();
                let name = TEST_NAMES[name_index];

                let divided_name = divider_clone.divide_name(&name.to_string());
                assert!(!divided_name.family.is_empty());
                assert!(!divided_name.given.is_empty());
            }
            println!("Thread {} with shared instance completed", thread_id);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
}

#[test]
fn test_gbdt_name_divider_separate_instances() {
    // 各スレッドで別々のGBDTNameDividerインスタンスを作成する場合
    // この方法は理論的にはスレッドセーフであるべき
    
    let mut handles = vec![];
    
    for thread_id in 0..5 {
        let handle = thread::spawn(move || {
            // 各スレッドで独立したdividerインスタンスを作成
            let divider = get_gbdt_name_divider(" ".to_string(), true, "gbdt".to_string());
            
            for i in 0..50 {
                let name_index = (thread_id * 50 + i) % TEST_NAMES.len();
                let name = TEST_NAMES[name_index];
                
                let divided_name = divider.divide_name(&name.to_string());
                assert!(!divided_name.family.is_empty());
                assert!(!divided_name.given.is_empty());
            }
            println!("Thread {} with separate instance completed", thread_id);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
}
