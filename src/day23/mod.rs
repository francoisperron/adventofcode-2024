mod lan_party;

#[cfg(test)]
mod tests {
    use crate::day23::lan_party::LanParty;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let lan_party = LanParty::from(EXAMPLE);

        assert_eq!(lan_party.connected(), 7);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(23);
        let lan_party = LanParty::from(&input);

        assert_eq!(lan_party.connected(), 1173);
    }

    #[test]
    fn solves_part2_example() {
        let lan_party = LanParty::from(EXAMPLE);

        assert_eq!(lan_party.password(), "co,de,ka,ta");
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(23);
        let lan_party = LanParty::from(&input);

        assert_eq!(lan_party.password(), "cm,de,ez,gv,hg,iy,or,pw,qu,rs,sn,uc,wq");
    }

    const EXAMPLE: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
}
