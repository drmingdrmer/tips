use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

/// # Safety
///
/// * `ptr` must be [valid] for writes.
/// * `ptr` must be properly aligned.
#[inline(always)]
pub unsafe fn store_advance_aligned<T>(val: T, ptr: &mut *mut T) {
    unsafe {
        std::ptr::write(*ptr, val);
        *ptr = ptr.add(1)
    }
}

#[inline(always)]
pub unsafe fn store_advance<T>(val: &T, ptr: &mut *mut T) {
    unsafe {
        std::ptr::copy_nonoverlapping(val as *const T, *ptr, 1);
        *ptr = ptr.add(1)
    }
}

// /// # Safety
// ///
// /// * `(ptr as usize - vec.as_ptr() as usize) / std::mem::size_of::<T>()` must be
// ///    less than or equal to the capacity of Vec.
#[inline(always)]
pub unsafe fn set_vec_len_by_ptr<T>(vec: &mut Vec<T>, ptr: *const T) {
    unsafe {
        vec.set_len((ptr as usize - vec.as_ptr() as usize) / std::mem::size_of::<T>());
    }
}

const N: usize = 1000;
const BATCH_SIZE: usize = 65536;

fn pointer_write() {
    // avoid LLVM
    let mut rng = rand::thread_rng();
    let random_idx: usize = rng.gen_range(0..BATCH_SIZE);

    for round in 0..N {
        let mut offsets: Vec<u64> = Vec::with_capacity(BATCH_SIZE);
        let mut offsets_ptr = offsets.as_mut_ptr();
        let mut data_size = 0;

        unsafe {
            for i in 0..BATCH_SIZE {
                data_size += (i + round) as u64;
                store_advance_aligned::<u64>(data_size, &mut offsets_ptr);
            }
            set_vec_len_by_ptr(&mut offsets, offsets_ptr);
        }
        assert_eq!(
            offsets[random_idx],
            ((random_idx + 1) * round + random_idx * (random_idx + 1) / 2) as u64
        );
    }
}

fn pointer_write_without_fn() {
    // avoid LLVM
    let mut rng = rand::thread_rng();
    let random_idx: usize = rng.gen_range(0..BATCH_SIZE);

    for round in 0..N {
        let mut offsets: Vec<u64> = Vec::with_capacity(BATCH_SIZE);
        let mut offsets_ptr = offsets.as_mut_ptr();
        let mut data_size = 0;

        unsafe {
            for i in 0..BATCH_SIZE {
                data_size += (i + round) as u64;
                std::ptr::write(offsets_ptr, data_size);
                offsets_ptr = offsets_ptr.add(1)
            }
            set_vec_len_by_ptr(&mut offsets, offsets_ptr);
        }
        assert_eq!(
            offsets[random_idx],
            ((random_idx + 1) * round + random_idx * (random_idx + 1) / 2) as u64
        );
    }
}

fn slice_get_unchecked() {
    // avoid LLVM
    let mut rng = rand::thread_rng();
    let random_idx: usize = rng.gen_range(0..BATCH_SIZE);

    for round in 0..N {
        let mut offsets: Vec<u64> = Vec::with_capacity(BATCH_SIZE);
        let mut data_size = 0;

        unsafe {
            let offsets = offsets.as_mut_slice();
            for i in 0..BATCH_SIZE {
                data_size += (i + round) as u64;
                *offsets.get_unchecked_mut(i) = data_size;
            }
        }
        unsafe {
            offsets.set_len(BATCH_SIZE);
        }
        assert_eq!(
            offsets[random_idx],
            ((random_idx + 1) * round + random_idx * (random_idx + 1) / 2) as u64
        );
    }
}

fn vec_get_unchecked() {
    // avoid LLVM
    let mut rng = rand::thread_rng();
    let random_idx: usize = rng.gen_range(0..BATCH_SIZE);

    for round in 0..N {
        let mut offsets: Vec<u64> = Vec::with_capacity(BATCH_SIZE);
        let mut data_size = 0;

        unsafe {
            for i in 0..BATCH_SIZE {
                data_size += (i + round) as u64;
                *offsets.get_unchecked_mut(i) = data_size;
            }
            offsets.set_len(BATCH_SIZE);
        }
        assert_eq!(
            offsets[random_idx],
            ((random_idx + 1) * round + random_idx * (random_idx + 1) / 2) as u64
        );
    }
}

fn vec_push() {
    // avoid LLVM
    let mut rng = rand::thread_rng();
    let random_idx: usize = rng.gen_range(0..BATCH_SIZE);

    for round in 0..N {
        let mut offsets: Vec<u64> = Vec::with_capacity(BATCH_SIZE);
        let mut data_size = 0;

        for i in 0..BATCH_SIZE {
            data_size += (i + round) as u64;
            offsets.push(data_size);
        }
        assert_eq!(
            offsets[random_idx],
            ((random_idx + 1) * round + random_idx * (random_idx + 1) / 2) as u64
        );
    }
}

fn pointer_copy() {
    // avoid LLVM
    let mut rng = rand::thread_rng();
    let random_idx: usize = rng.gen_range(0..BATCH_SIZE);

    for round in 0..N {
        let mut offsets: Vec<u64> = Vec::with_capacity(BATCH_SIZE);
        let mut offsets_ptr = offsets.as_mut_ptr();
        let mut data_size = 0;

        unsafe {
            for i in 0..BATCH_SIZE {
                data_size += (i + round) as u64;
                store_advance::<u64>(&data_size, &mut offsets_ptr);
            }
            set_vec_len_by_ptr(&mut offsets, offsets_ptr);
        }
        assert_eq!(
            offsets[random_idx],
            ((random_idx + 1) * round + random_idx * (random_idx + 1) / 2) as u64
        );
    }
}

fn extend_from_into_iter() {
    // avoid LLVM
    let mut rng = rand::thread_rng();
    let random_idx: usize = rng.gen_range(0..BATCH_SIZE);

    for round in 0..N {
        let mut offsets: Vec<u64> = Vec::with_capacity(BATCH_SIZE);
        let mut data_size = 0;

        offsets.extend((0..BATCH_SIZE).into_iter().map(|i| {
            data_size += (i + round) as u64;
            data_size
        }));

        assert_eq!(
            offsets[random_idx],
            ((random_idx + 1) * round + random_idx * (random_idx + 1) / 2) as u64
        );
    }
}

fn extend_from_iter() {
    // avoid LLVM
    let mut rng = rand::thread_rng();
    let random_idx: usize = rng.gen_range(0..BATCH_SIZE);

    for round in 0..N {
        let mut offsets: Vec<u64> = Vec::with_capacity(BATCH_SIZE);
        let mut data_size = 0;

        offsets.extend((0..BATCH_SIZE).map(|i| {
            data_size += (i + round) as u64;
            data_size
        }));

        assert_eq!(
            offsets[random_idx],
            ((random_idx + 1) * round + random_idx * (random_idx + 1) / 2) as u64
        );
    }
}

fn bench_vec(c: &mut Criterion) {
    c.bench_function("pointer_write", |b| b.iter(|| pointer_write()));
    c.bench_function("pointer_write_without_fn", |b| {
        b.iter(|| pointer_write_without_fn())
    });
    c.bench_function("slice_get_unchecked", |b| b.iter(|| slice_get_unchecked()));
    c.bench_function("vec_get_unchecked", |b| b.iter(|| vec_get_unchecked()));
    c.bench_function("pointer_copy", |b| b.iter(|| pointer_copy()));
    c.bench_function("extend_from_into_iter", |b| {
        b.iter(|| extend_from_into_iter())
    });
    c.bench_function("extend_from_iter", |b| b.iter(|| extend_from_iter()));
    c.bench_function("vec_push", |b| b.iter(|| vec_push()));
}

criterion_group!(benches, bench_vec);
criterion_main!(benches);
