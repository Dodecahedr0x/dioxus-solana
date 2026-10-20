//! Display helpers shared by most Solana UIs.

/// Lamports per SOL.
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

/// Format lamports as SOL, trimming trailing zeros (`1_500_000_000` → `"1.5"`).
pub fn format_sol(lamports: u64) -> String {
    let whole = lamports / LAMPORTS_PER_SOL;
    let frac = lamports % LAMPORTS_PER_SOL;
    if frac == 0 {
        return whole.to_string();
    }
    let frac = format!("{frac:09}");
    format!("{whole}.{}", frac.trim_end_matches('0'))
}

/// Parse a decimal SOL amount (`"1.5"`) into lamports; `None` if malformed or
/// more precise than a lamport.
pub fn parse_sol(sol: &str) -> Option<u64> {
    let sol = sol.trim();
    let (whole, frac) = sol.split_once('.').unwrap_or((sol, ""));
    if (whole.is_empty() && frac.is_empty()) || frac.len() > 9 {
        return None;
    }
    let digits = |s: &str| s.is_empty() || s.bytes().all(|b| b.is_ascii_digit());
    if !digits(whole) || !digits(frac) {
        return None;
    }
    let whole: u64 = if whole.is_empty() {
        0
    } else {
        whole.parse().ok()?
    };
    let frac: u64 = if frac.is_empty() {
        0
    } else {
        format!("{frac:0<9}").parse().ok()?
    };
    whole.checked_mul(LAMPORTS_PER_SOL)?.checked_add(frac)
}

/// Shorten an address or signature to `head…tail` (4 characters each side).
pub fn short_address(address: &impl ToString) -> String {
    let s = address.to_string();
    if s.len() <= 10 {
        return s;
    }
    format!("{}…{}", &s[..4], &s[s.len() - 4..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sol_round_trips() {
        assert_eq!(format_sol(1_500_000_000), "1.5");
        assert_eq!(format_sol(2 * LAMPORTS_PER_SOL), "2");
        assert_eq!(format_sol(1), "0.000000001");
        assert_eq!(parse_sol("1.5"), Some(1_500_000_000));
        assert_eq!(parse_sol(".25"), Some(250_000_000));
        assert_eq!(parse_sol("3"), Some(3 * LAMPORTS_PER_SOL));
        assert_eq!(parse_sol("0.0000000001"), None);
        assert_eq!(parse_sol("1.2.3"), None);
        assert_eq!(parse_sol("abc"), None);
        assert_eq!(parse_sol(""), None);
    }

    #[test]
    fn shortens_addresses() {
        assert_eq!(
            short_address(&"MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            "Memo…fcHr"
        );
        assert_eq!(short_address(&"short"), "short");
    }
}
