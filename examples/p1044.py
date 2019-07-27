class Solution(object):
    def longestDupSubstring(self, S):
        """
        :type S: str
        :rtype: str
        """

        def has_equal(a, bs, span):
            for b in bs:
                same = True
                for i in range(span):
                    if S[a+i] != S[b+i]:
                        same = False
                        break
                if same:
                    return True
            return False

        ords = [0] * len(S)
        for i, c in enumerate(S):
            ords[i] = ord(c) - ord('a')
            # ords[i] = hash(c) % (2 ** 31 - 1)

        span_low, span_high = 0, len(S)
        res = -1
        mod = 2**63 - 1
        while span_low + 1 < span_high:
            span = (span_low + span_high + 1) / 2
            # print(span, span_low, span_high)
            p = pow(26, span, mod)
            start = -1
            record = {}
            num = 0
            for n in ords[:span]:
                num = (num * 26 + n) % mod
            # num = reduce(lambda x, y: (x * 26 + y) % mod, ords[:span], 0)
            # for i in range(span):
            #     num += ords[i]
            record[num] = [0]
            for i in range(1, len(S) - span + 1):
                num = (num * 26 + ords[i+span-1] - ords[i-1] * p) % mod
                # cur = (cur * 26 + A[i] - A[i - L] * p) % mod
                # num -= ords[i-1]
                # num += ords[i+span-1]
                if num in record:
                    if has_equal(i, record[num], span):
                        start = i
                        break
                    record[num].append(i)
                else:
                    record[num] = [i]
            if start == -1:
                span_high = span
            else:
                span_low = span
                res = start

        # print("xxx", res, span_low, span_high)
        if res == -1:
            return ""
        return S[res:res+span_low]

if __name__ == '__main__':
    s = Solution()
    # print(s.longestDupSubstring("banana"))
    # print(s.longestDupSubstring("abcd"))
    print(s.longestDupSubstring("moplvidmaagmsiyyrkchbyhivlqwqsjcgtumqscmxrxrvwsnjjvygrelcbjgbpounhuyealllginkitfaiviraqcycjmskrozcdqylbuejrgfnquercvghppljmojfvylcxakyjxnampmakyjbqgwbyokaybcuklkaqzawageypfqhhasetugatdaxpvtevrigynxbqodiyioapgxqkndujeranxgebnpgsukybyowbxhgpkwjfdywfkpufcxzzqiuglkakibbkobonunnzwbjktykebfcbobxdflnyzngheatpcvnhdwkkhnlwnjdnrmjaevqopvinnzgacjkbhvsdsvuuwwhwesgtdzuctshytyfugdqswvxisyxcxoihfgzxnidnfadphwumtgdfmhjkaryjxvfquucltmuoosamjwqqzeleaiplwcbbxjxxvgsnonoivbnmiwbnijkzgoenohqncjqnckxbhpvreasdyvffrolobxzrmrbvwkpdbfvbwwyibydhndmpvqyfmqjwosclwxhgxmwjiksjvsnwupraojuatksjfqkvvfroqxsraskbdbgtppjrnzpfzabmcczlwynwomebvrihxugvjmtrkzdwuafozjcfqacenabmmxzcueyqwvbtslhjeiopgbrbvfbnpmvlnyexopoahgmwplwxnxqzhucdieyvbgtkfmdeocamzenecqlbhqmdfrvpsqyxvkkyfrbyolzvcpcbkdprttijkzcrgciidavsmrczbollxbkytqjwbiupvsorvkorfriajdtsowenhpmdtvamkoqacwwlkqfdzorjtepwlemunyrghwlvjgaxbzawmikfhtaniwviqiaeinbsqidetfsdbgsydkxgwoqyztaqmyeefaihmgrbxzyheoegawthcsyyrpyvnhysynoaikwtvmwathsomddhltxpeuxettpbeftmmyrqclnzwljlpxazrzzdosemwmthcvgwtxtinffopqxbufjwsvhqamxpydcnpekqhsovvqugqhbgweaiheeicmkdtxltkalexbeftuxvwnxmqqjeyourvbdfikqnzdipmmmiltjapovlhkpunxljeutwhenrxyfeufmzipqvergdkwptkilwzdxlydxbjoxjzxwcfmznfqgoaemrrxuwpfkftwejubxkgjlizljoynvidqwxnvhngqakmmehtvykbjwrrrjvwnrteeoxmtygiiygynedvfzwkvmffghuduspyyrnftyvsvjstfohwwyxhmlfmwguxxzgwdzwlnnltpjvnzswhmbzgdwzhvbgkiddhirgljbflgvyksxgnsvztcywpvutqryzdeerlildbzmtsgnebvsjetdnfgikrbsktbrdamfccvcptfaaklmcaqmglneebpdxkvcwwpndrjqnpqgbgihsfeotgggkdbvcdwfjanvafvxsvvhzyncwlmqqsmledzfnxxfyvcmhtjreykqlrfiqlsqzraqgtmocijejneeezqxbtomkwugapwesrinfiaxwxradnuvbyssqkznwwpsbgatlsxfhpcidfgzrc"))
    print(s.longestDupSubstring("bedddcdebeeabbaebbeeecbdaeebdeecbcaccdadbbebbcceeaabcaeaabbcabbeaaaabbbebcdabcacecdeaccbaedadcebddcecdbbddddcaebbacdbeeaecdebdacbcecebeaaaecbeebcddecabbdcbcaaacbcbcdeddeebeddecbccbbdeecdccbebdeccadabaaccaccbcabcdadaecbcbebbdacbcddeddaceecaebaaebdceaabbbbddedcbddceebcddbccceabbccbedaaebddabcbdeebdceabacdabbcbaceecbcaedeaabcecbbcdcbecbadbceeaddcdebabebdcdacebddccaeeabecbbddbeeeeabcaedddcaaeeeabdeddbabecaeebcbdabadceeebbaaeeaedeceaabccebddeeebcbaeebabdabcdbeedecbccedebcbbdaabbecacdcbdddeddaaeaaedcaecabaaeebbedeedcaaedbabcabbcccbbadeeabcdcecadabcaeeadcbeebaaaebdadcdeebdbeeeecdbbdecadbebaacabaeaabdaedcdbdaecaaadbbaabaecdebcbeeddbedbabdbbbedadaeecdbeebbbbadcddbedcadecedcdaaedeedeeebddbedaaeebababbdedcedebbdbeaccaeeaccdbacbdbaccdaedabcbabbcaeeddadeebecaeccaabacebabceaaaecaccbbcbbbddaaaeaeeacccbabcdcbcbbbddccabedbdceacbdbeaacabdaadbbcdacbacbeeaccecdbcbcadaddcdddddacaeeeeedeedbecdacbdcccdaabcebeacabbeccaccaddebeaadaedcadaecebddcabecaceebbabbbeeaadcacaaeeacadedbdcadbeacedcbabacdecbbdbbbdceecdbaedebbcedcdebbabdcabeecdaeaccecdedddabcadedebbbebedabbdadeaeeddbbceabbbbbebecbbcdbdeabbbacbaeedecbebbecbabaabaabeaceddaeaccccaddabbacdcdccbbecbcaebdabddaebecdedccddacbaedbdbeddccadcaaeaaaeaebbacdbedccdbcebbadcdbdaaeadbcacccaeadeecdeedbceebdcbabbdbebddcaeeedebbebbdbbdbdaceadabeabecededaddcaadeccebbdcdaedabdadecccdbcedbcdeeaaecabbcecbecdbabdaabdbdacaaccdeddbdbbeeeeaeeedaacddcedabdbbdaabdcdabbeeeeebbbcdaceeaccbdedebacadbebebecdcebbeadabccbedeadcdcacdcbacecebbbeebadddbdddaaeaececeaeecbdbdcbbcbdbcdceadeaabdcdcccdccdeeacadebcdecededbaeabdcceaebaeaddbeaaacedabcecedececdabdebebddaaccdabeeedaeadbabdbcbcedecbecdecbbcbbbcdeaccdedbcbbaeccdeacecabdedbadadddebdcdacddcbaacdabdddbccdddbebbcebbbbcbbbbdbdabdaaeceadedbaedadcabbaeaebddbcdeebcaebccdeaadceeacddabdcabdbbcadabbccabdbbaaaacbbbeaaeaaeeebdcaaeccbcbacdbbdcdcededbaedccdaadeaccdbeaadabbcccbbddaaaebadeabdaaacecceadacbaedbdedbcaebccaebcdadbbedbaccebacadabbedcddbcedddbedeeaedecaccaabdaeaadecdaccabebebbccebebaaaedabcddcbdeabbedaecdcbbcccbcebebbdccccdadeddbeaaccbdaeccebbacebccabbdbeacecbabddbbbecbcebebddadeaedeadbbdceacabdaeceaaeecacbbecdecebbcccbdbbdccdccebbbdaaccbdbbecbacdbaccaeedadcbecebbaaeaeeaabcceebccecaabceacdbaadccdcedeeeaadacbbeecaeebbddcacddcadcddcbecdbbbaddacaacdabbcecdecadecbaaceecbdebbdcbabacabeaacedceececcadcbcdbebcacaaaeccbbcecbbbecadcbababdaccaedcbaacaeabdcdcddbebbdeaeaaeedceedeabdcccbaaddcabcecedaccbdeccceacaacaddabddddebedeeccdcaecbbcdaadbeadddbeecdddabbedbddaeaaeecdccdddbdcacbdbdaabeeadecbbbbaccdaceccdcceeebdcaedceadbdcdddbceaadacceddbadbeebdbdbddaeadddbdebacecabbbddadaacdcdccdbadccdeedaebbdcbceebcacdaeaadeacbdadbdebbbbbaeacdaecabeeeadbbddebcddcedeeaacbabbdacdaeddbabececeedeeeabccaccabbbedcbeadbcceaccceadbdaabbdebeeaeedcebbacaaaadceddbabaebeeddcbdeecccbadecabcecbcedebdadcdddedbeaebadcbcdcdbecabcdaeeabbecaacccdeebeeeccebbabdaccecbeceadbddacabecdbaeceecccededbddccaedccdcebdecdaddbedeaeaecbadbcebcddbaedaaddccaaabccbeaabebbdcceedacebbacddeaeadaabadddbcdddcdbaeeeadbbeacbeebadbdbadeedbccdadcebadbebaabdbaeeededdeddcdeeaacacceaeaebebdceedadddabbbdddecabbcbdebaeaaeaabadeaaddacacbebbdeabdceaecdbbaeadebaeddcdcaeddaacdabcaebdaedddbaebcdbabeededdbbbcedcbbabdbaeaabbcbbaaedddbdbdcabdaaccacebaddccbcbcbdccedaaeedcccbbddbeeeedccbaaeedaacccdccadeebcbbeeeeababbcadcadbadedaacbbedecdabacbebcbeaeacdabbaebabeeabcbeabcbbddbacedbdacbdbabedeeebbbdbacdecdceaeecaedeabbdbdaaebcbbccceacbaadebbcbceeeadadccbbdaadcbcecabbadabeacedcbaebdddecdeebcbbadcbaaaacbccbecabbcbbbaacedaedbadceebdbcdaaadadbebcdcaccedaeededebaddedbacbbddeeecaebcddebcddcceeecdaebdecccecbbebcacaaedcecadcbdaecbebaadbacdbceecccaedccbbddaeedebacdaccbedabaaadbddcabedcedaaadaeecedddbdcbccacedbdadeecedeadebaebccccdaeccebcbdaecbeedcddeddabcdaccdbecebcdebbbbbdbcbbeddbdadbbcceaaedeeedcabddddacbbddbddebcddbdabaabadbaddbdabdadcbccadeaeccadbecedbbadcdcedaceaaacadcdebcccbeaacdeebbbcbddcaadcaeddaccbebcbbbebbdeeedadbdabeacbcddaddcacacbdbdeadbdadbdedebdcadcebbccaedeebcdcbdeaeedcbadeebeaedddcbddbaeaeeeebbcbaeccceecdaecdbdacebeadeeaddcebabccaacaadabeeacbacbdebedabbacabceebbadcdccbacbcadbbbadadabaadbdacbcccdecdcceeeaebadddcbceeebeeecbebcdaecccbdcbebdcdecdbeabacadaccebbbdbaedadbedcbeeeaeccaeecabbecabaceacbeccbbbddcdbeedaecdeabaecbdcaddcacdbeeccdaedecbdacebdabdeeeebbbeadccecbdebedaadcbeddceceacecbecdaabceadeeadedeeeabdaabeacdeacadbbbedbdbbadababcbacabbcbcabcbbacdacbacbccdbddeebccdedebaccaaadccbddcedddabacccbdeabeaaecaabddeeacadeecebeddaebaebbeedeaecacadbedeebdacbdecdaecedbcabdadddebabcabbbdbddbaecaeccdbcbdaddaeebbadeecbbeebecaeaabddedadccdacbacdcbcaaaeeacaccdedaaeeebedecadcacedeadedcdaaebbecedabaebeedbbbbbcbddbeadcadcbdacebdaaabbbadeddcaadeadbddedaadaababeabeccceadaeedbcdcdbadcaccceacbddcceeddadeeddaddebbaecbbaeddebeaabbeccaedacdcdbcbcccbaacecaaacddaebbeeaeaabebacecdabadceedeaaceaddceeedaeacbbadcaeddcdbccddececaebbcbcbeccdaaddcdbbdedaadacdbbabbbcbbcdbaddcaaddeeabecbcecbcdbbcbbeeddeedecceceeadcdccbeecbddadbccdabbdceaaeeadaededabdcbccddaabcaeddeacaaabdcceedebbeadddddcdbdbcdbddeecaacdccbddecacdaadedcaeeedbbcecaeacbcdddbabcdedccbbddebcbacbeebdbbcedadbceecddcbbacbeeadbadcbceebadabbddeddbecabdddabedccdbedcaebdeebbbacdeebecebcadabcceeeabeebddcdcaedececcecdbaacceeccadecaacbaecdeecbebeaadcdbdedaabdcbedaecddcbcbdbbeaeaecbaeabddaeccaabbccdbbeaebeabcaaccecaeadcdccaeeccbedbebadbecdccaaaadbeacaacbbbcdacbeeceaceceaadbdccdcbaecebbebcebdbcdbabaecbdaacbbdacdeecbdbdacecdaeecccccaccaabccebbeaaceeaebbbceeaaeedeacdcbbcbeacaedeeacacdbadeebcaecceadaadddbaaaaaccadedccbccdaaaabaccdedddaeeeaadddddabaadcbeabcdbcabdebdedaccbdaaaabbccedaebdccdeeabdbccbdaaaaabbbdddaaecdececcdabdddebddadceaedcaddeabcdbacdcdeadeebdecdbaebdcbdadeceecdbccbaceeebdbbeacaabbeedbccdaddccbababebbdbcddceccbccccbdcebeadedeeaceadabbccbebadbabeeaebcbbeebcbdadadabadedabddbeebddcdedbedbabdaaaabdedbddccdcbdacedbbeadecbadacbcecbcdaceeaaaebdaebaeeacebccbcbedddeeaeacadaecccaecdcbbaabcdebebbdbdeadcadecdaebddbedbddeeddccbddadcecdccdadaadabdbeaedadbccbceaabebeaeaeacddecbddcccbdacdcaddbaccbaecceeccaeceadeeaaddaddaaaabcebececcebdddecacbebdcbcbeeebbabbcebcbdcabbdaeaacbbbebeacabbaaaabedcebbbbadcebccccaeddcbededdbcabeeedbcabbbbdaccceadaeccacaaeececaeeaebcccdccedabeaddababbcaecaddbcaccdacdcadbddcdcbcbdbaadbcdddadabeececbaddebbeadbcdaebececcabaacbcaddadacebbebebcebeabebaedbdcddebdaeeccbdeedceceabeadbecbdacdcaabcdaeedcebbdabbeacccebbaadaadecddbbeadbccdadadcdbcadbcddeaaaabeaedccdcccacdaeccccaecddbeddbceabdbecdddcacdbaebcedccaacbdeeabddedceacebeeadadecbbaccbacbdaebecbdbacecbbedaeabddbeeedcdbbcceecccedcbbeebcbeaeadedeeedabdcdcbbcbcccaadbebbabecbdcceadeccceddacabdbcdbbecbcededaccbbbebbdccaecddbcdecdcbdddbcdacbbcbdabdacbbebadbaedebeeeabdbbdbdbaeecbbcebbeacddcdabccbcbccbceebcbaadaaedcdccceaaebeaeccabcdeeeaacaaecadaeeaedebaecbcacedccaaeaccddeaaabdcedbdecabeaabaccaaabbbceaeccedeacdcbabbdaadebdbdcbcbeddcaaccabeaccdedcebcbdccbabcdcadececcbcdbceabbebcdaecddacaabceabcdbedddecabbcbaceedbdbacbebbeaddcabbdbedadbaeecdeedbdbbebbdcaadcbbaadbdedbdddacbccbdbbedbaddddedcdbdabecacccdcbabddbdbedbbcababbbdaccbcdbdcdecdecceacacbecadecbabbdacccebeeaabeedbadeccccebaeabcdeaacbbccbeeabbaddbeaadccdcaaadceeaeaaddddbedacbeecaaaabedcacabdadcdeacaccacbcabcadcedbeaebbbaecaebeeaddaecdceecbcdbbabdddcdccdedbcadabbbacacabbabaaeebddaaebcbdaeaeadacaedaaaaedeeedcbdaaeeaadaaecddedacbebddabbeddbddcdaebccdcebbaceeecabaadcbccbaddbdacbaddceecdbcecaabdededaaebcdaeccaaebeabaeddabecceaecaeeabdbdbddaaeeebaedecbadaeeaddbcedadcaeacabbdebacaabaeddeedbaaededaadecccdceccababbeeceededbcbdbdaadcdaabdcbeaaeaedcebcbaccabbdeceecdeacccabcaddbeaceadeaecacaeaddbdbdeeeedacdcecbacdceeadababdcadbdaddcbccbaeabebeddbebaccbaaacbaaaebebdcccdbacbbddedacbdbddbbbaaecbaedaebbbdabedbccadebedcbeaedcbaabddcebdccaeaceebddcadcabddadebbaccabeeeaccdaacdeacdabccadecbbcecbbabeabddccbedebaceedbceddcaedabcdeedcdbbbeebaadbcdcedcacabedddbcacabeadbbbbcaeeacecadabbdcbdacdbccbbedecadaebebeaecedabdcbbaaadbdabbdabacbeecabbcadeaecbedaccccdbbcbaeeacbcccddadcdeaaacdbdccbbeedbddedaaddecaeedbcdccdecbaadddbeddbcddcecccbeaecdbbcbcbecdebbedbccbbcaadddbcaabdcdbbeededcaddbbecbeceaccdacccabbeaebbbdeaabbcbecdaccdacaebcedcedeadbdababadaebacacdaebcaaebcedecdeabcedeacedabdcbaeedeeceaaaddecbeecbbeeadaebdacbbbbacccedabdaebcdaaecbeaaebddbdddabccaeebbcdcacbacacadbcebdabcdccdcbebeeedcbdeddeeeeaacebdacadecbbbadbbcbbcbabcacccdadeccbdcbbebbdeabeedeeccacaabcebcbbadecdaaddeadedebbacbcaebebbcaadddbbeecaaaecbdccbcacaadedeeddacedebbebdeecbedaddccdbdcaecaccabeaaacedccaddacedcebdceeebbebcaacaecbeadedeebbadbdcaceaaeabbbcbdcbebbeeeedcacecaeeadcbbeeeadddddbcbdededcddedcaebbcdaecbbdceedacdaeedbeeaaacdbcddceaebeecbceadbecaaccbacbcdbacacdecadddadebdbbdbeeeaacebddaddcdeaebbcabaadcbeadaabcecadadbdcbdededcdeacdbacbebebcbbdbdebababbecebbbcbdebeacedbccbaaabeceeeacacbbdcbbbccdeeaecbaeaaebeccecccaadaaeddeaceeedeeececbeaadccedcdaeeeaedaebeddcdacdacbbccdecbdaaebacdacbebaedaeecbbdbeedcccadbabacdeebdeacdbaaedcebbeeaebcedbecccddcdcdeebbbeceacbdcaacbadaccbedbcbcdebacadcbcbbbcbcebdcddbcbaeaabcdcbdceceedecdcceeeabebaabcdbaedeedabcdbaabceaedecbeebdcbaaddaccbddcbdcebacaecbabdabcceeaecedbeaaadaaebcabdaededdadcdaabacbdceddcebdeceeeeedeeedecbeaceaeceddbbbedebdebddcbdcdeaceacedacdbaebedcbcaebadbcdcadabaeebbbccababcaaedabacecbedadaaddbcabdcccacabcabbdebbdbccbbaeeeaeedccdddbacbeaddbeddceecbabccbabeeeaacbcdcaeccecacbdeddeceabbeeeebdcabcbcbcbcbcdbbcecdbabbceddccebbaacadbcbcaedbeececadcdbabbdbddddebdeeeaddaddedabcebdabbcdadbdeaecdebcaaccddaccadabdeddcbaedacdcecebbcccaedababacbabdbeaceeceedeeaceebddcddddcbceedaceaaaecdbcadbdbcecbcdcaaeeeeaadeecbbdbcacdcdeceebcbdcebbdebaeabdddeaebaaaddbeddabaddaedcacceeceebedcebcaedaeacedacdeadaaacedaeabacceeccabbbdcbecacebaeedccacbbdcaeaeacdbebcacaaccbeebcbdcceccadabacdcddcbcdbacebcaedaebcbdeeceddccadacdeadeaabeeaddbdcdbdeddeaeddbcadbcabceccbccddbcabccebcebdebbbaecdbabaabedeaeacdbbbedacddedeebacbacacdbebebeaeedbccabdceccdbbaecadedcabacbdadbeceadeabdcdbbbeeddcaeadddbdceacddabcdeebedaaabbbcaeaaedbeebaaeccdbeaadeaddddeeeeceacbdcceededeacdabbabcbbbbcdccddedaaebcabaababddcdeecbeaceadceabcaceabbeaacaccecbddbedeecdbabdcedbbabaddbcadbdaeeeedabadddeedabdebcedddcecacccdebdadedbbdaccbcbdacaabeccaeebccdbebebeaadacbdbcacebeebddadebbacbaeddeeaecaedcbddbceadedbbbdaebbacbaeedeecacaabebdbbacbbddcbdeeccacdebdeeccdbcdaabcacdcdacaacdaccbecaebaecbabcbceecdbdeaeebcadbaeedbaeecbacceaacbbbecbeecedaabcdeaeabaaebcaedbbbcaabddddeccdedbedaaaceeacacaaadacadcbaeccecbaddababccbbcdedcabcacedacebaceeacaadaaeebceabaebebeabdeabcbbadacdbeccbabddbbadbbbbaabeeddbcbcbebeeedecaeccbbddbeccbbabcdbadbecdeeeaaabdbcbcaeabadaeeccdbeeddccccbeedebcdcccacacebbbebabeeecdcdbadedeacacaadeadacdeceecabedaaaaaaddcbdeaeadaaebdebdacaeaeeddabdabeaadbbbadbcbcabedbacebabdcbcecbebaeddcccaacbeabeabaedbbbddabccecbbebacebaccbebcaeedbbaaebabebdedacdbabbbeeebacbbcbaddadacbecbcdebcccaaeacbdedcceaedeeeaccbeaeebdbcbbebdeebdcdceacebcdbacaadaabecabeeeaaacedadbbdcaeeceadebddebdeadeaedcadbeaeaeeddeedeeacebbdeccdcbdaedececdaaecacccaaccaddddcdeadddeabdecadbabbedceaccaeacdccbdbaabbdbcadceabbdebdadaebbaabedaddecabbdcaacdcacbbdbddcecadbedeaededaddadbbbeadacbbcbeeeebaacdeabddadbbdaabbadeabcbcaaaceeddcaddcacbaabcbcdcaceadaacedacdbbcabedcbacedeedbdddbeeabceebcacbacaddadacacebbaeccaeebcabcdbdaedcabbedacaeddbbacebecbeedeaaedecbcbbcecdeeacadadeacbadbdddbccdabeaeaaeeaadadbdeebbcdebcddaaccaeddbacccaacabbdabcdabeebdabdebbdbcbbcbeddcccdaeccebeeaadddbcbbabeeeadebddccaebdabbdcddcbdadcdbeecaaebbecbdbbedcedacdceabbbbdbbeadaaddbdcadaabacdeeecccebdbaababbacdcdadaedcdacdbcbbebbacabbbbdacbeecceaabdcbaacdebdbdededbddcabadbbaceadaeecbdddadcebedbeabeeaacebccbeaabcacdecceeabaeceecbdbdcbdbbadeeccedaedbaeddadcadccebaaadadddacdadaaedbcbcceccacaaaedcaeecdececdddaedabdbebcdedceadbacbaeededeccdeecbaececcddeaddbebdbeebbdcdaabbebecbacdbeccdcebcbcdaaeacaedecbcdcdcdaceedcdbdeaeeeaaabbdcbceacaebedeaadadaeacedcbddadddaeaccaaddaeccabaabeaddbdcbacbddceeaecbbbaedbcacaeebccebdcaebdcbcdeeedbeacdddbecabcdcbbaadabbdecedbaccdbabdccacaeabecbcbaccaecebadcdccccbbedacadecaaeaeacbdddaadcaeabcaaadadaebadbeecdbdebcdbccccaaadccaeedecadbceccecceaaadbeadddccbaccebcedeaededcdcdaabdeedbabcdecabaebadeddacaddccbadccdbabbdaaaddcabdeaeddbedadaaaecebadaaeaabbcebacceebababbbadcaabcddaadcdccdbaccbdbbcbcdddccedcadcdabcbbcebbacaceedecaccaeaebabcdbdbeccbbbbcaedeeebcadebeedccddedccdaadaacedcbbaddbddbacbdacdbbccdedbddaddbacadcceeadcbdbaceebbadbbdbcbadebcebedacacbccedbcabbccadabacbbcaecabdedcaadedaaecaacbcecdbddddabbaceceadeaebaaeecdbbdbaedbacebabaaadbcbdcaadbebacdaacbeacdceebaaccdbbdbeacacbccaaacedbcbcabbdeceaedacbceacccedbabcedebabbecbebbcabeebcecebbdbbeeeabcbdeadcccabcdbcbbaebddbbeacbbcbcbbaebedeeecddaeddcbebeababbcabacbbacbbdeaebbdacaeeeadcaeaebdbccbabdecccbeeabdcddddabcdbdcddaeaabeebaeaedadccebddcaedabadddeccddbbabccadceddadbaeaecbadcebbdbecebeabadceeddbbaeecbddcebcbbbebbabaadacbaaecebabbabeacacadaebcbedacedbddcdbdebebacbdebacabaedcddeebbcbaadabeeaacbededdbdcbeadaedeabeabaceeddcdaadecbadecabeaadedbcbbbadadddabbebdacaaccaedcdbcbecdbdecacbbedceababcbedcbedbeccabddbcbbedebbaeedbecadbbaacbeedabcdaeabbaaaebaadcecbacbcceaedeccabbdbbcdceabeaebdcaaeaae"))