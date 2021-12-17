fn main()
{
    println!("Part 1: {}", part1(raw_input()));
    println!("Part 2: {}", part2(raw_input()));
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: [bool; 3],
    data: Data,
}

#[derive(Debug, PartialEq)]
enum Data {
    Literal(PacketLiteral),
    Operator(PacketOperator),
}

#[derive(Debug, PartialEq)]
struct PacketLiteral {
    value: u64,
}

#[derive(Debug, PartialEq)]
struct PacketOperator {
    operator: [bool; 3],
    packets: Vec<Packet>,
}

fn part1(hexa: &'static str) -> u64
{
    let binary: Vec<bool> = hexa.chars().flat_map(hexa_to_binary).collect();

    let packet = parse_packet(&binary).0;

    compute_sum(packet)
}

fn compute_sum(packet: Packet) -> u64
{
    let mut sum = binary_to_decimal(&packet.version);

    if let Data::Operator(operator) = packet.data {
        for packet in operator.packets {
            sum += compute_sum(packet);
        }
    }

    sum
}

fn part2(hexa: &'static str) -> u64
{
    let binary: Vec<bool> = hexa.chars().flat_map(hexa_to_binary).collect();

    let packet = parse_packet(&binary).0;

    compute_value(&packet)
}


fn compute_value(packet: &Packet) -> u64
{
    match &packet.data {
        Data::Literal(literal) => literal.value,
        Data::Operator(operator) => {
            let operator_value = binary_to_decimal(&operator.operator);

            match operator_value {
                0 => operator.packets.iter().map(|packet| compute_value(packet)).sum(),
                1 => operator.packets.iter().map(|packet| compute_value(packet)).product(),
                2 => operator.packets.iter().map(|packet| compute_value(packet)).min().unwrap(),
                3 => operator.packets.iter().map(|packet| compute_value(packet)).max().unwrap(),
                5 => {
                    assert_eq!(2, operator.packets.len());

                    let first = compute_value(&operator.packets[0]);
                    let second = compute_value(&operator.packets[1]);

                    if first > second {
                        1
                    } else {
                        0
                    }
                },
                6 => {
                    assert_eq!(2, operator.packets.len());

                    let first = compute_value(&operator.packets[0]);
                    let second = compute_value(&operator.packets[1]);

                    if first < second {
                        1
                    } else {
                        0
                    }
                },
                7 => {
                    assert_eq!(2, operator.packets.len());

                    let first = compute_value(&operator.packets[0]);
                    let second = compute_value(&operator.packets[1]);

                    if first == second {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!(),
            }
        },
    }
}

fn parse_packet(bits: &[bool]) -> (Packet, &[bool])
{
    let split = bits.split_at(3);
    let version = parse_3(split.0);
    let mut tail = split.1;

    let split = tail.split_at(3);
    let type_id = parse_3(split.0);
    tail = split.1;

    if type_id == [true, false, false] {
        let mut value_bits = vec![];
        loop {
            let split = tail.split_at(1);
            let should_continue = split.0[0];
            tail = split.1;
            
            let split = tail.split_at(4);
            let value = split.0;
            tail = split.1;

            value_bits.extend_from_slice(value);
            if ! should_continue {
                break;
            }
        }

        (Packet { version, data: Data::Literal(PacketLiteral { value: binary_to_decimal(&value_bits) }) }, tail)
    } else {
        let split = tail.split_at(1);
        tail = split.1;

        let packets = match split.0[0] {
            false => {
                let split = tail.split_at(15);
                let value = binary_to_decimal(split.0);
                tail = split.1;

                let split = tail.split_at(value as usize);
                tail = split.1;
                let mut packets = vec![];
                let mut sub_tail = split.0;
                loop {
                    let result = parse_packet(sub_tail);
                    let packet = result.0;
                    sub_tail = result.1;
                    
                    packets.push(packet);

                    if sub_tail.is_empty() {
                        break;
                    }
                }

                packets
            },
            true => {
                let split = tail.split_at(11);
                let value = binary_to_decimal(split.0);
                tail = split.1;

                let mut packets = vec![];
                for _ in 0..value {
                    let (packet, sub_tail) = parse_packet(tail);
                    packets.push(packet);
                    tail = sub_tail;
                }

                packets
            },
        };

        (Packet { version, data: Data::Operator(PacketOperator { operator: type_id, packets }) }, tail)
    }
}

fn parse_3(bits: &[bool]) -> [bool; 3]
{
    [bits[0], bits[1], bits[2]]
}

fn binary_to_decimal(bits: &[bool]) -> u64
{
    let mut binary = 0;
    for (index, value) in bits.iter().rev().enumerate() {
        if *value {
            binary += 2_u64.pow(index as u32);
        }
    }

    binary
}

fn hexa_to_binary(char: char) -> [bool; 4]
{
    match char {
        '0' => [false, false, false, false],
        '1' => [false, false, false, true],
        '2' => [false, false, true, false],
        '3' => [false, false, true, true],
        '4' => [false, true, false, false],
        '5' => [false, true, false, true],
        '6' => [false, true, true, false],
        '7' => [false, true, true, true],
        '8' => [true, false, false, false],
        '9' => [true, false, false, true],
        'A' => [true, false, true, false],
        'B' => [true, false, true, true],
        'C' => [true, true, false, false],
        'D' => [true, true, false, true],
        'E' => [true, true, true, false],
        'F' => [true, true, true, true],
        _ => panic!(),
    }
}

fn parse_binary_string(string: &'static str) -> Vec<bool>
{
    string.chars().map(|char| {
        match char {
            '1' => true,
            '0' => false,
            _ => panic!(),
        }
    }).collect()
}

#[test]
fn it_works()
{
    let bits = parse_binary_string("110100101111111000101000");
    let (packet, tail) = parse_packet(&bits);
    assert_eq!(&[false, false, false], tail);
    assert_eq!(6, binary_to_decimal(&packet.version));

    assert_eq!(Data::Literal(PacketLiteral { value : 2021 }), packet.data);

    assert_eq!(16, part1("8A004A801A8002F478"));
    assert_eq!(12, part1("620080001611562C8802118E34"));
    assert_eq!(23, part1("C0015000016115A2E0802F182340"));
    assert_eq!(31, part1("A0016C880162017C3686B18A3D4780"));

    assert_eq!(3, part2("C200B40A82"));
    assert_eq!(54, part2("04005AC33890"));
    assert_eq!(7, part2("880086C3E88112"));
    assert_eq!(9, part2("CE00C43D881120"));
    assert_eq!(1, part2("D8005AC2A8F0"));
    assert_eq!(0, part2("F600BC2D8F"));
    assert_eq!(0, part2("9C005AC2F8F0"));
    assert_eq!(1, part2("9C0141080250320F1802104A08"));
}

fn raw_input() -> &'static str
{
    "20546718027401204FE775D747A5AD3C3CCEEB24CC01CA4DFF2593378D645708A56D5BD704CC0110C469BEF2A4929689D1006AF600AC942B0BA0C942B0BA24F9DA8023377E5AC7535084BC6A4020D4C73DB78F005A52BBEEA441255B42995A300AA59C27086618A686E71240005A8C73D4CF0AC40169C739584BE2E40157D0025533770940695FE982486C802DD9DC56F9F07580291C64AAAC402435802E00087C1E8250440010A8C705A3ACA112001AF251B2C9009A92D8EBA6006A0200F4228F50E80010D8A7052280003AD31D658A9231AA34E50FC8010694089F41000C6A73F4EDFB6C9CC3E97AF5C61A10095FE00B80021B13E3D41600042E13C6E8912D4176002BE6B060001F74AE72C7314CEAD3AB14D184DE62EB03880208893C008042C91D8F9801726CEE00BCBDDEE3F18045348F34293E09329B24568014DCADB2DD33AEF66273DA45300567ED827A00B8657B2E42FD3795ECB90BF4C1C0289D0695A6B07F30B93ACB35FBFA6C2A007A01898005CD2801A60058013968048EB010D6803DE000E1C6006B00B9CC028D8008DC401DD9006146005980168009E1801B37E02200C9B0012A998BACB2EC8E3D0FC8262C1009D00008644F8510F0401B825182380803506A12421200CB677011E00AC8C6DA2E918DB454401976802F29AA324A6A8C12B3FD978004EB30076194278BE600C44289B05C8010B8FF1A6239802F3F0FFF7511D0056364B4B18B034BDFB7173004740111007230C5A8B6000874498E30A27BF92B3007A786A51027D7540209A04821279D41AA6B54C15CBB4CC3648E8325B490401CD4DAFE004D932792708F3D4F769E28500BE5AF4949766DC24BB5A2C4DC3FC3B9486A7A0D2008EA7B659A00B4B8ACA8D90056FA00ACBCAA272F2A8A4FB51802929D46A00D58401F8631863700021513219C11200996C01099FBBCE6285106"
}