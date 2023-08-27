
//! ### CP/M Packing Module
//! 
//! Functions to help pack or unpack dates, filenames, and passwords.
//! N.b. CP/M passwords are stored with a trivial encryption algorithm
//! and should not be considered secure.

use chrono::{Timelike,Duration};
use log::{debug,info,warn,error};
use std::str::FromStr;
use super::types;
use crate::DYNERR;
const RCH: &str = "unreachable was reached";

pub fn pack_date(time: Option<chrono::NaiveDateTime>) -> [u8;4] {
    let now = match time {
        Some(t) => t,
        _ => chrono::Local::now().naive_local()
    };
    let ref_date = chrono::NaiveDate::from_ymd_opt(1978, 1, 1).unwrap()
        .and_hms_opt(0,0,0).unwrap();
    let days = match now.signed_duration_since(ref_date).num_days() {
        d if d>u16::MAX as i64 => {
            warn!("timestamp is pegged at {} days after reference date",u16::MAX);
            u16::to_le_bytes(u16::MAX)
        },
        d if d<0 => {
            warn!("date prior to reference date, pegging to reference date");
            [0,0]
        }
        d => u16::to_le_bytes(d as u16 + 1)
    };
    let hours = (now.hour() / 10)*16 + now.hour() % 10;
    let minutes = (now.minute() / 10)*16 + now.minute() % 10;
    return [days[0],days[1],hours as u8,minutes as u8];
}

pub fn unpack_date(cpm_date: [u8;4]) -> chrono::NaiveDateTime {
    let ref_date = chrono::NaiveDate::from_ymd_opt(1978, 1, 1).unwrap();
    let now = ref_date + Duration::days(u16::from_le_bytes([cpm_date[0],cpm_date[1]]) as i64 - 1);
    let hours = (cpm_date[2] & 0x0f) + 10*(cpm_date[2] >> 4);
    let minutes = (cpm_date[3] & 0x0f) + 10*(cpm_date[3] >> 4);
    return now.and_hms_opt(hours.into(), minutes.into(), 0).unwrap();
}

/// Take string such as `2:USER2.TXT` and return (2,"USER2.TXT")
pub fn split_user_filename(xname: &str) -> Result<(u8,String),DYNERR> {
    let parts: Vec<&str> = xname.split(':').collect();
    if parts.len()==1 {
        return Ok((0,xname.to_string()));
    } else {
        if let Ok(user) = u8::from_str(parts[0]) {
            if user<types::USER_END {
                return Ok((user,parts[1].to_string()));
            } else {
                error!("invalid user number");
                return Err(Box::new(types::Error::BadFormat));
            }
        }
        error!("prefix in this context should be a user number");
        return Err(Box::new(types::Error::BadFormat));
    }
}

/// Accepts lower case, case is raised by string_to_file_name
pub fn is_name_valid(s: &str) -> bool {
    let it: Vec<&str> = s.split('.').collect();
    if it.len()>2 {
        return false;
    }
    let base = it[0];
    let ext = match it.len() {
        1 => "",
        _ => it[1]
    };

    for char in [base,ext].concat().chars() {
        if !char.is_ascii() || types::INVALID_CHARS.contains(char) || char.is_ascii_control() {
            debug!("bad file name character `{}` (codepoint {})",char,char as u32);
            return false;
        }
    }
    if base.len()>8 {
        info!("base name too long, max 8");
        return false;
    }
    if ext.len()>3 {
        info!("extension name too long, max 3");
        return false;
    }
    true
}

/// put the filename bytes as an ASCII string, result can be tested for validity
/// with `is_name_valid`
pub fn file_name_to_string(name: [u8;8],typ: [u8;3]) -> String {
    // in CP/M high bits are explicitly not part of the name
    let base: Vec<u8> = name.iter().map(|x| x & 0x7f).collect();
    let ext: Vec<u8> = typ.iter().map(|x| x & 0x7f).collect();
    [
        &String::from_utf8(base).expect(RCH).trim_end(),
        ".",
        &String::from_utf8(ext).expect(RCH).trim_end(),
    ].concat()
}

/// put the filename bytes as a split ASCII string (name,type)
pub fn file_name_to_split_string(name: [u8;8],typ: [u8;3]) -> (String,String) {
    // in CP/M high bits are explicitly not part of the name
    let base: Vec<u8> = name.iter().map(|x| x & 0x7f).collect();
    let ext: Vec<u8> = typ.iter().map(|x| x & 0x7f).collect();
    (   String::from_utf8(base).expect(RCH).trim_end().to_string(),
        String::from_utf8(ext).expect(RCH).trim_end().to_string()
    )
}

/// put the filename bytes as an ASCII string with hex escapes
pub fn file_name_to_string_escaped(name: [u8;8],typ: [u8;3]) -> String {
    // in CP/M high bits are explicitly not part of the name
    let base: Vec<u8> = name.iter().map(|x| x & 0x7f).collect();
    let ext: Vec<u8> = typ.iter().map(|x| x & 0x7f).collect();
    let base_str = crate::escaped_ascii_from_bytes(&base, true, false);
    let ext_str = crate::escaped_ascii_from_bytes(&ext, true, false);
    match ext_str.trim_end().len() {
        0 => base_str.trim_end().to_string(),
        _ => [base_str.trim_end(),".",ext_str.trim_end()].concat()
    }
}

/// Convert string to name and type bytes for directory.
/// Assumes string contains a valid filename.
pub fn string_to_file_name(s: &str) -> ([u8;8],[u8;3]) {
    let mut ans: ([u8;8],[u8;3]) = ([0;8],[0;3]);
    let upper = s.to_uppercase();
    let it: Vec<&str> = upper.split('.').collect();
    let base = it[0].as_bytes().to_vec();
    let ext = match it.len() {
        1 => Vec::new(),
        _ => it[1].as_bytes().to_vec()
    };
    for i in 0..8 {
        if i<base.len() {
            ans.0[i] = base[i];
        } else {
            ans.0[i] = 0x20;
        }
    }
    for i in 0..3 {
        if i<ext.len() {
            ans.1[i] = ext[i];
        } else {
            ans.1[i] = 0x20;
        }
    }
    return ans;
}

/// Accepts lower case, case is raised by string_to_password
pub fn is_password_valid(s: &str) -> bool {
    for char in s.chars() {
        if !char.is_ascii() || types::INVALID_CHARS.contains(char) || char.is_ascii_control() {
            debug!("bad password character `{}` (codepoint {})",char,char as u32);
            return false;
        }
    }
    if s.len()>8 {
        info!("password too long, max 8");
        return false;
    }
    true
}

/// Convert password to (decoder,encrypted bytes) for directory.
/// Assumes string contains a valid password.
pub fn string_to_password(s: &str) -> (u8,[u8;8]) {
    // assumes is_password_valid was true;
    let mut ans: (u8,[u8;8]) = (0,[0;8]);
    let decrypted = s.to_uppercase().as_bytes().to_vec();
    for i in 0..8 {
        let delta = match i<decrypted.len() { true => decrypted[i], false => 0x20 };
        ans.0 = ((ans.0 as u16 + delta as u16) % 256) as u8;
    }
    for i in 0..8 {
        if i<decrypted.len() {
            ans.1[7-i] = ans.0 ^ decrypted[i];
        } else {
            ans.1[7-i] = ans.0 ^ 0x20;
        }
    }
    return ans;
}
