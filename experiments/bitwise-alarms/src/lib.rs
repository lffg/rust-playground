pub const AVAILABLE_ACTIONS: &[&str] = &[
    "Ataque Magnético",
    "Status da Válvula",
    "Vazamento no Pulso 2",
    "Corte de Cabo do Pulso 2",
    "Vazamento no Pulso 1",
    "Corte de Cabo do Pulso 1",
];

pub fn parse_alarm(hex: &str) -> impl Iterator<Item = &'static str> {
    let num = u32::from_str_radix(hex, 16).unwrap();
    AVAILABLE_ACTIONS
        .iter()
        .enumerate()
        .filter_map(move |(index, &action)| (num & 1 << index != 0).then(|| action))
}

#[cfg(test)]
mod tests {
    macro_rules! test {
        ($name:ident, ($input:expr, $radix:expr), $out:expr) => {
            #[test]
            fn $name() {
                let parsed = u32::from_str_radix($input, $radix).unwrap();
                let hex_string = format!("{:x}", parsed);
                let out = super::parse_alarm(&hex_string).collect::<Vec<_>>();
                assert_eq!(out, Vec::from($out));
            }
        };
    }

    test!(t1, ("00000001", 2), ["Ataque Magnético"]);
    test!(t2, ("00000010", 2), ["Status da Válvula"]);
    test!(t3, ("00010000", 2), ["Vazamento no Pulso 1"]);
    test!(t4, ("00100000", 2), ["Corte de Cabo do Pulso 1"]);
    test!(
        t5,
        ("00100001", 2),
        ["Ataque Magnético", "Corte de Cabo do Pulso 1"]
    );
}
