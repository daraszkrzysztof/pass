use std::ops::{RangeInclusive, Range};
use std::error::Error;
use crate::database::Database;
use std::time::{Instant};
use rayon::prelude::*;
use rayon::iter::ParallelIterator;

pub use super::charset::Charset;

#[derive(Debug)]
struct BruteforceParams {
    len_range: RangeInclusive<usize>,
    charset: Charset,
}

pub fn bruteforce() -> Result<(), Box<dyn Error>> {
    let params = BruteforceParams {
        len_range: 4..=8,
        charset: "abcdefghijklmnopqrstuvwxyz0123456789".into(),
    };

    println!("{:?}", params);

    let records =
        Database::with(|db| Ok(db.records.clone()))?;

    let start_time = Instant::now();

    // for len in params.len_range.clone() {
    //     params.charset
    //         .range(len as _)
    //         .into_par_iter()
    //         .for_each_with(vec![0u8; len], |mut buf, i| {
    //             params.charset.get_into(i, &mut buf);
    //             let hash = md5::compute(&buf);
    //
    //             for (db_user, db_hash) in &records {
    //                 if &hash.to_vec() == db_hash {
    //                     println!("[CRACKED in {:?}] user ({}) has password ({})",
    //                              start_time.elapsed(),
    //                              db_user,
    //                              std::str::from_utf8(&buf).unwrap_or("<not utf-8>")
    //                     );
    //                 }
    //             }
    //         })
    // }

    let mut r_count = records.capacity();
    'outer: for len in params.len_range {
        let mut buf = vec![0u8; len];
        for i in params.charset.range(buf.len() as _) {
            params.charset.get_into(i, &mut buf);
            let hash = md5::compute(&buf);

            for (db_user, db_hash) in &records {
                if &hash.to_vec() == db_hash {
                    println!("[CRACKED in {:?}] user ({}) has password ({})",
                             start_time.elapsed(),
                             db_user,
                             std::str::from_utf8(&buf).unwrap_or("<not utf-8>")
                    );
                    r_count = r_count - 1;
                    if r_count == 0 {
                        println!("All password cracked!");
                        break 'outer;
                    }
                }
            }
        }
    }

    Ok(())
}

