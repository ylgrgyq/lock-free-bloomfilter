use bloomfilter::BloomFilter;
use std::time::SystemTime;

#[test]
fn test_filter() {
    let test_size = 1000000;
    println!("starting to test bloom filter, {} keys", test_size);

    let mut f1 = BloomFilter::create(test_size, 0.000001);
    let now = SystemTime::now();
    let result = (0..test_size)
        .into_iter()
        .map(|n| f1.set(&n.to_string()))
        .filter(|b| !*b)
        .count();
    let n2 = SystemTime::now();
    let d = n2.duration_since(now).unwrap();
    println!("test size {}, true {} in {}.{} secs",
             test_size,
             result,
             d.as_secs(),
             d.subsec_nanos());

    let now = SystemTime::now();
    let result = (0..test_size)
        .into_iter()
        .map(|n| f1.might_contain(&n.to_string()))
        .filter(|b| !*b)
        .count();
    let n2 = SystemTime::now();
    let d = n2.duration_since(now).unwrap();
    println!("test size {}, true {} in {}.{} secs",
             test_size,
             result,
             d.as_secs(),
             d.subsec_nanos());

    let test_size = 100000;
    println!("starting to test bloom filter, {} keys", test_size);

    let mut f1 = BloomFilter::create(test_size, 0.001);
    let now = SystemTime::now();
    let result = (0..test_size)
        .into_iter()
        .map(|n| f1.set(&n.to_string()))
        .filter(|b| !*b)
        .count();
    let n2 = SystemTime::now();
    let d = n2.duration_since(now).unwrap();
    println!("test size {}, true {} in {}.{} secs",
             test_size,
             result,
             d.as_secs(),
             d.subsec_nanos());
}