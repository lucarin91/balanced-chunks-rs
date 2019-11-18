extern crate balanced_chunks;

use balanced_chunks::BalancedChunksExt;
use balanced_chunks::BalancedChunksMutExt;

#[test]
fn vec_1chunk() {
    let v = vec![1, 2, 3, 4, 5];
    let mut chunks = v.balanced_chunks(1);
    assert_eq!(chunks.next().unwrap(), [1, 2, 3, 4, 5]);
}

#[test]
fn vec_2chunk() {
    let v = vec![1, 2, 3, 4, 5];
    let mut chunks = v.balanced_chunks(2);
    assert_eq!(chunks.next().unwrap(), [1, 2, 3]);
    assert_eq!(chunks.next().unwrap(), [4, 5]);
}

#[test]
fn vec_3chunk() {
    let v = vec![1, 2, 3, 4, 5];
    let mut chunks = v.balanced_chunks(3);
    assert_eq!(chunks.next().unwrap(), [1, 2]);
    assert_eq!(chunks.next().unwrap(), [3, 4]);
    assert_eq!(chunks.next().unwrap(), [5]);
}

#[test]
fn vec_4chunk() {
    let v = vec![1, 2, 3, 4, 5];
    let mut chunks = v.balanced_chunks(4);
    assert_eq!(chunks.next().unwrap(), [1, 2]);
    assert_eq!(chunks.next().unwrap(), [3]);
    assert_eq!(chunks.next().unwrap(), [4]);
    assert_eq!(chunks.next().unwrap(), [5]);
}

#[test]
fn vec_5chunk() {
    let v = vec![1, 2, 3, 4, 5];
    let mut chunks = v.balanced_chunks(5);
    assert_eq!(chunks.next().unwrap(), [1]);
    assert_eq!(chunks.next().unwrap(), [2]);
    assert_eq!(chunks.next().unwrap(), [3]);
    assert_eq!(chunks.next().unwrap(), [4]);
    assert_eq!(chunks.next().unwrap(), [5]);
}

#[test]
fn vec_1chunk_mut() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut chunks = v.balanced_chunks_mut(1);
    assert_eq!(chunks.next().unwrap(), [1, 2, 3, 4, 5]);
}

#[test]
fn vec_2chunk_mut() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut chunks = v.balanced_chunks_mut(2);
    assert_eq!(chunks.next().unwrap(), [1, 2, 3]);
    assert_eq!(chunks.next().unwrap(), [4, 5]);
}

#[test]
fn vec_3chunk_mut() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut chunks = v.balanced_chunks_mut(3);
    assert_eq!(chunks.next().unwrap(), [1, 2]);
    assert_eq!(chunks.next().unwrap(), [3, 4]);
    assert_eq!(chunks.next().unwrap(), [5]);
}

#[test]
fn vec_4chunk_mut() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut chunks = v.balanced_chunks_mut(4);
    assert_eq!(chunks.next().unwrap(), [1, 2]);
    assert_eq!(chunks.next().unwrap(), [3]);
    assert_eq!(chunks.next().unwrap(), [4]);
    assert_eq!(chunks.next().unwrap(), [5]);
}

#[test]
fn vec_5chunk_mut() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut chunks = v.balanced_chunks_mut(5);
    assert_eq!(chunks.next().unwrap(), [1]);
    assert_eq!(chunks.next().unwrap(), [2]);
    assert_eq!(chunks.next().unwrap(), [3]);
    assert_eq!(chunks.next().unwrap(), [4]);
    assert_eq!(chunks.next().unwrap(), [5]);
}
