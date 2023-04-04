use std::collections::HashSet;

fn is_start_seq(chunk: &[u8]) -> bool {
    let bit_0 = chunk[0];
    let bit_1 = chunk[1];
    let bit_2 = chunk[2];
    let bit_3 = chunk[3];

    if bit_0 == bit_1 || bit_0 == bit_2 || bit_0 == bit_3 {
        return false;
    }

    if bit_1 == bit_2 || bit_1 == bit_3 {
        return false;
    }

    if bit_2 == bit_3 {
        return false;
    }

    return true;
}

#[test]
fn test_is_start_seq_01() {
    let chunk: &[u8] = "abcd".as_bytes();
    assert_eq!(is_start_seq(chunk), true);
}

#[test]
fn test_is_start_seq_02() {
    let chunk: &[u8] = "aacd".as_bytes();
    assert_eq!(is_start_seq(chunk), false);
}

fn find_start_of_packet_marker(s: &String) -> usize {
    for i in 0..s.len() - 3 {
        let chunk: &[u8] = &s[i..i + 4].as_bytes();
        let chuck_is_start: bool = is_start_seq(chunk);
        if chuck_is_start {
            return i + 4;
        }
    }
    panic!("no start of packet found")
}

#[test]
fn test_find_start_of_packet_marker_01() {
    let input: String = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
    let index_start_packet = find_start_of_packet_marker(&input);
    assert_eq!(index_start_packet, 5)
}

#[test]
fn test_find_start_of_packet_marker_02() {
    let input: String = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
    let index_start_packet = find_start_of_packet_marker(&input);
    assert_eq!(index_start_packet, 6)
}

#[test]
fn test_find_start_of_packet_marker_03() {
    let input: String = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
    let index_start_packet = find_start_of_packet_marker(&input);
    assert_eq!(index_start_packet, 10)
}

#[test]
fn test_find_start_of_packet_marker_04() {
    let input: String = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
    let index_start_packet = find_start_of_packet_marker(&input);
    assert_eq!(index_start_packet, 11)
}

fn is_start_of_message_marker(chunk: &[u8]) -> bool {
    let mut hashset: HashSet<u8> = HashSet::new();
    for j in chunk {
        hashset.insert(*j);
    }
    if hashset.len() == 14 {
        return true;
    } else {
        return false;
    }
}

#[test]
fn test_is_start_of_message_marker_01() {}

fn find_start_of_message_marker(s: &String) -> usize {
    for i in 0..s.len() - 14 {
        let chunk: &[u8] = &s[i..i + 14].as_bytes();
        let chuck_is_start: bool = is_start_of_message_marker(chunk);
        if chuck_is_start {
            return i + 14;
        }
    }
    panic!("no start of packet found")
}

#[test]
fn test_find_start_of_message_marker_01() {
    let input: String = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
    let index_start_packet = find_start_of_message_marker(&input);
    assert_eq!(index_start_packet, 19)
}

#[test]
fn test_find_start_of_message_marker_02() {
    let input: String = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
    let index_start_packet = find_start_of_message_marker(&input);
    assert_eq!(index_start_packet, 23)
}

#[test]
fn test_find_start_of_message_marker_03() {
    let input: String = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
    let index_start_packet = find_start_of_message_marker(&input);
    assert_eq!(index_start_packet, 23)
}

#[test]
fn test_find_start_of_message_marker_04() {
    let input: String = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
    let index_start_packet = find_start_of_message_marker(&input);
    assert_eq!(index_start_packet, 29)
}

#[test]
fn test_find_start_of_message_marker_05() {
    let input: String = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
    let index_start_packet = find_start_of_message_marker(&input);
    assert_eq!(index_start_packet, 26)
}

fn main() {
    let input: String = "sbpbwwrlwrwggscsfcsshsvhshphttzctztfflddvbvcbcvbcbnbrrstsspsvpvllncccqssqdssnsmshmshhtssbzzlplffzppddmhhwnhntnjtnjtnnqggjdgjdjrjffsdfsfbsffhtthwwfssqpspllcvcdvvfvfzfbzzgpglpgpmppfsfdfbfvvfjfsjjvqqbvqbvqbvvlvglvglvljvjzvvqcvqcqjcqqpddqbqcqrqmmvnvbnnfqnncznccqjcqqjzzwlltrrmmlwlllbjlblzlddfdmmscmsspfssqggbsbsjjdqdqgqgqbqdqrqlrrltttghthrtrwrdrnrfnfvftvtsvtvwtvwttvqtvtccqtqmqggwhhhlzhhrwwbwqwbbfrrmddwhdwdnwdwbdwwsswtwnnvvdggbbtwbbwllgffqpffpgpgmpmmjqqmpmffjgfgrffdzzspzptzzdszszbsbsbsvsbbhsbsbddnhhrqqcwwblbwlwnnthtsshmshmhgmgqgjqqpccfvcvbbbnllgmlgmmbnmnlmlzlffrrrgssmcmddnpdndtntrrqdqldldbbtppvddgndnwwctwccpbpffngnsgshggphggtssgngtnthtllzflzflzfzrfftgtstppghgnnpggrdgdnnswwccljlflwwgzgcgrrhssbwbllblnlvlddsffgrrnbbmsmmmhjmjvmjvmjmqqpdqppltlflmfmssqcqsqvsvsggpglplslggqtggmsmmmgffrccrvvsdvdvfddprdprrwjrjcjjmlmrmqmggqgdqdwdsdwwwgfwfddrcdcvcmmjbbmrrhlrrwcwvcclttwbwttgffnmnjmjdmmmlqmqrrlblplccdbcbwcbcpphpqpqlqrllqwwjsjgjbjqbbmcmzmrrqtrqqmcmmnjmjbbtnnbbjdjzzvbzvzjzdjzjrzzfzfbbmnmdnnzppmbpbhbdbvvqnqqnqrnrzrhhrddbqdqpqqwgwlglplglslhhpjpdjdjgddnppmvppjddczccgsccdsccbcvczccnbnmmsnsmmrwmwbmbbcpbbwbppzggdnnnzmnzzvrvcrvcvcrvrnrzzqzmzttjhhnffqggnqqwzwfzfwzwfwcwnccwgcwgcgqcgqcggzgwgfwgfffjttjddwswqqnttqwqsqhhcmchcscbbnwnlwlrlppqnnwjnjtntrtdtbbmzzrmzrzvrzvrzzwqwqllccnffvmmfmvmzzfttnnzttrbttdvvhdhzhzhvzzfsfszsvsttrqqdlldflfjjnqjjbhhqhzhcchppzsznntdntdddqqwjqjqmmgnnhrhjhnjnznwwdpplzzfzztbzzbmzmbbgjgtjtqjqpqhqgqhhtztpzpmzmjmhhjwwjsspbssvdsvsgvssghgvglgtltgllmhmtmhttfgglwgllpvpgpspzzhnnzcnzzldzdmzmlzzwtwhhlfhhvthhjdjsdsmddgzzsjjnznjnmjjgdgtdtzdtdvtddzwzgwwqvqccwrrrvddbtdbbshbhssnvsnvssswcwjcjlltvvltvvdsdqdsswlwttzfzhzjjgjsgjgttbccsvcscllbfbzzwwwznwwtzwzjzwjjgqjqrjrdjrddzsznsznzpzszhzzspzzbzrzhhsbspsqpqqqpddhdcdldttlccjrjsrsfftfdfrrtntpnpsnnwjnwnznpzzlqqlmqlmmcddlqqzssglsglssprmnvltqhslvqmvszjtvtwqjcdngjmftnhwvjdvtwwhtnsdmvjdspnhnlmjgnmwlspcvpdmlsrnbbzlmwwrslssmcbggmfvgzsnpnlnzdqsbhcfjdccrspnzfmhbvwstvccvqqjlwhpnlrrwszjnrtdfzwrwlzwvdvbzbvltdpfwrjlslmrctwvbbvdrctgtgwtwpjjghhvdsqhplfrsjqlgsrbfgwdjlzdpdljtvjmpwqqbghndqnvjhngtpnpvzfbtchncwdqjhmzjlpdggbdcqrfjlwvczvpspljqmpgtrsvwwhqncfvfrwbnvsfjqlsjdlrqzmlqjgcpghhgzfhjcglllnhtmchrrptbzhqnfntgqbfstrvpsqsvqvcvpgjnchbvmgtgzqfrqcjrvldzghdfrvllrtfcwnsmmgdrcbjmdqgbfwmpwhfjmnbqrbvqvnmjlqtsjqvhzpdsgbjpjngmzbgnznvjqprfvwzjrfrwfdqrtlcgqqlvrzwmqwjbdvprvpwvdcrfdrcttnmnvjfrsrrmjgfjdmpdpnfsrwnprtdpvdmdwssvjrqtlcvpgrqgqqqffvvssbmghzjrzrzlcrnfjtdbvwsjzfvcrsmgqbcrdrjwdwbltffwbgjwtgtdblmlhvhlcpgdmpcztmpmgjghqpwzwtpnmnmgnqqrrtwczgmgtdbgdqtpnlbnzhshsfzsmrztffrmlsgqcprbjpqwjlqgwvctpmpshgzbzsjgqhzvsrjfwplvjbvltrlfldvsmlppcmsfrbbctggmqmjnhppstzrcfjtdgfwrrnmlvjphwtlqtjqcntjtzvgjtwvthjfbgpwhlrzdqncmggvgthmgjvrbnzbndsldnlcgtcqbqdnnbnqhvtpfnrclttfwcpnqscjbzdvbqrrsbzpdfhjllbjwsltjpmdnbrrzvhvzzqnlbglsjrnjbqffnnzmldfvtvmldfsztrnpcjgblsdhzfzmfqzlfrtslglhfvszppptjnjqcdmjcwqmfzhnqbfhslwhvtjfvcftzsphvghvtjjswpwghnfngmzddbwwqddsphvhcrwtthsjfswfqbvdsqghmrspdldfqmchnnrcdvjsclcnlsncsplchvzrwqbtvvqlqspftrwmjcbgpzbsmnfccbzgnhqsfjgmmsqsscdscfjbrmmtjbsphhlrlsgbllrptqrcgnqchzfddjwlldsbpcnzfbspfpchclqfbbtjpmtmtjthcdvwhrtqbgmgldcgcnmmhtbnqpzzcwlrscbzcqjzgztwjrnbmsnqtcllznlctzrntftspmnvhtfwbljnmrwsstvbmwclqrfpmwvjphrwddzdwtlfcvzlvqdmzhnvslfnfjhvdndlgbvvzbztpwvqzbzsbtqpqfmgqfgpzctfrqfjwmsnmlfqbgrlmncntcbshjhdcbqnvznhtcgcmmhnsbwpzbvtqbntwgflhjgmvvfhdbwfqmnfjlzdvvnpmvjrdfdnrhpbtllhbtbswwvrbwjgnqpbgnfrjtvbczbpmrcbwdlhztzssnwshjmmcqchptrtchrqncdgdtmwrlnmmwqlzqswwwvpngvwcphgnzrhpprjnbldscvwlqdjwnhjrnscdwlnhnsbwgzjtgvzdgqcjcgvrdhntszhdnjsbbrfphlmdlldjdslbjnnsfbmcnvtlczmtnhrwblnbrdptcpmsbwqptgmwzsqnmmchwnnrvrlfsrglfzzqbnzmpdtnhhbmfqvsrsdsctvhqwfgbtvhbbrsrqmrvvplrnbfnbdmrvzpgctdtglndhcqnllvvcppgfbwjrpqcbghlqdbmpzwrqpmvwddqgthlmzmdsvzdfsmgzltbsvphctzgjsmqvgjlsbgnvmgprbcsfhgrtbwtnnrsqcwfzrhlgjcwcfrjhffrvrvtnpczvwvjnnhfdgcppnnjjpttptcbmdqvgdbhdmlqgcqsrnbcrbtcgzbvgmhbnwzsgnwzbhdqqmvtpssvlvsttgnmcclqnjcgjnvtdggrcwsgbpjljgzgtllsnfvfshtbbpwrjhzvzswlfdvhbpngvgddcmhbzqcvnjhfsqpnvvsdvdtmqlqpzcgsnwlflnqprbqnwdqchjvsptbtrvtzvhrmrvznfpzmcsgnqtdvghhzwrrwvqwrztvdbjjtfchpftdcbthpfdczwchpptwzdpswvbhppdphgvpfzhprpqtnprgfmdnqrbrdlclcmhrdfrcdhwpcqhnbwmhrrgnctpvsqmphcwwvlmslszhdz".to_string();
    let index_start_packet: usize = find_start_of_packet_marker(&input);
    let index_start_message: usize = find_start_of_message_marker(&input);
    println!("start of packet {:#?}", index_start_packet);
    println!("start of message {:#?}", index_start_message);
}
