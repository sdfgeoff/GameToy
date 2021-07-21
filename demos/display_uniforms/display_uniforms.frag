// This shade encodes the BedStead pixel-font and displays it.

const ivec2 char_space = ivec2(0, 0);
const ivec2 char_exclam = ivec2(-2147352576, 135274560);
const ivec2 char_quotedbl = ivec2(0, 338186240);
const ivec2 char_numbersign = ivec2(-494600192, 338229411);
const ivec2 char_dollar = ivec2(-2058944512, 475277538);
const ivec2 char_percent = ivec2(1179385856, 105660480);
const ivec2 char_ampersand = ivec2(-1572143104, 68429858);
const ivec2 char_quoteright = ivec2(0, 135270400);
const ivec2 char_parenleft = ivec2(1090781184, 269488160);
const ivec2 char_parenright = ivec2(16842752, 68173953);
const ivec2 char_asterisk = ivec2(-985530368, 139751489);
const ivec2 char_plus = ivec2(-2130706432, 1057264);
const ivec2 char_comma = ivec2(16908800, 0);
const ivec2 char_hyphen = ivec2(0, 224);
const ivec2 char_period = ivec2(131072, 0);
const ivec2 char_slash = ivec2(1077936128, 4210752);
const ivec2 char_zero = ivec2(578945024, 136874258);
const ivec2 char_one = ivec2(-2130247680, 135798848);
const ivec2 char_two = ivec2(1078951936, 474251456);
const ivec2 char_three = ivec2(71761920, 1044398274);
const ivec2 char_four = ivec2(-503054336, 271601811);
const ivec2 char_five = ivec2(71761920, 1040480514);
const ivec2 char_six = ivec2(608632832, 403179762);
const ivec2 char_seven = ivec2(1082195968, 1044398144);
const ivec2 char_eight = ivec2(608632832, 474253538);
const ivec2 char_nine = ivec2(33751040, 474253794);
const ivec2 char_colon = ivec2(131072, 8192);
const ivec2 char_semicolon = ivec2(16908800, 8192);
const ivec2 char_less = ivec2(1090781184, 269488144);
const ivec2 char_equal = ivec2(-536870912, 63491);
const ivec2 char_greater = ivec2(16842752, 68174081);
const ivec2 char_question = ivec2(-2147352576, 474234944);
const ivec2 char_at = ivec2(-1605959680, 474278227);
const ivec2 char_A = ivec2(-465010688, 136874259);
const ivec2 char_B = ivec2(608665600, 507807986);
const ivec2 char_C = ivec2(608632832, 474220560);
const ivec2 char_D = ivec2(608665600, 507808018);
const ivec2 char_E = ivec2(542081024, 1040451824);
const ivec2 char_F = ivec2(541097984, 1040451824);
const ivec2 char_G = ivec2(609157120, 474220563);
const ivec2 char_H = ivec2(608731136, 574917106);
const ivec2 char_I = ivec2(-2130247680, 470818880);
const ivec2 char_J = ivec2(71761920, 541098242);
const ivec2 char_K = ivec2(-1572306944, 572794928);
const ivec2 char_L = ivec2(542081024, 33818640);
const ivec2 char_M = ivec2(608731136, 577546578);
const ivec2 char_N = ivec2(608731136, 574921043);
const ivec2 char_O = ivec2(608632832, 474253586);
const ivec2 char_P = ivec2(541097984, 507807984);
const ivec2 char_Q = ivec2(-1572143104, 474253586);
const ivec2 char_R = ivec2(-1572306944, 507807984);
const ivec2 char_S = ivec2(71761920, 474220770);
const ivec2 char_T = ivec2(-2130575360, 1041244224);
const ivec2 char_U = ivec2(608632832, 574916882);
const ivec2 char_V = ivec2(1090650112, 574916769);
const ivec2 char_W = ivec2(-1522204672, 574916946);
const ivec2 char_X = ivec2(1145602048, 574902337);
const ivec2 char_Y = ivec2(-2130575360, 574902336);
const ivec2 char_Z = ivec2(1078951936, 1044398144);
const ivec2 char_bracketleft = ivec2(1083113472, 1007161376);
const ivec2 char_backslash = ivec2(67108864, 266305);
const ivec2 char_bracketright = ivec2(34045952, 505430145);
const ivec2 char_asciicircum = ivec2(0, 136873984);
const ivec2 char_underscore = ivec2(1015808, 0);
const ivec2 char_quotereversed = ivec2(0, 135282688);
const ivec2 char_a = ivec2(-1001455616, 28931);
const ivec2 char_b = ivec2(608665600, 33847570);
const ivec2 char_c = ivec2(542048256, 61456);
const ivec2 char_d = ivec2(609157120, 541126930);
const ivec2 char_e = ivec2(-532217856, 28947);
const ivec2 char_f = ivec2(-2130575360, 269492448);
const ivec2 char_g = ivec2(609161244, 61714);
const ivec2 char_h = ivec2(608731136, 33847570);
const ivec2 char_i = ivec2(-2130247680, 134230080);
const ivec2 char_j = ivec2(-2130574332, 134225984);
const ivec2 char_k = ivec2(-1031208960, 67670176);
const ivec2 char_l = ivec2(-2130247680, 202383424);
const ivec2 char_m = ivec2(-1521844224, 22866);
const ivec2 char_n = ivec2(608731136, 30994);
const ivec2 char_o = ivec2(608632832, 28946);
const ivec2 char_p = ivec2(608665858, 30994);
const ivec2 char_q = ivec2(609161248, 61714);
const ivec2 char_r = ivec2(1082195968, 53344);
const ivec2 char_s = ivec2(-1006141440, 61457);
const ivec2 char_t = ivec2(-2130444288, 135295040);
const ivec2 char_u = ivec2(609157120, 35090);
const ivec2 char_v = ivec2(1115815936, 35089);
const ivec2 char_w = ivec2(-1522204672, 35090);
const ivec2 char_x = ivec2(-2104983552, 34976);
const ivec2 char_y = ivec2(609161244, 35090);
const ivec2 char_z = ivec2(-2138079232, 63616);
const ivec2 char_braceleft = ivec2(-2129920000, 806363168);
const ivec2 char_brokenbar = ivec2(-2130575360, 135274496);
const ivec2 char_braceright = ivec2(-2130608128, 101720192);
const ivec2 char_asciitilde = ivec2(0, 72630272);
const ivec2 char_filledbox = ivec2(-405831680, 1048377843);
const ivec2 char_sterling = ivec2(1083146240, 407375984);
const ivec2 char_quotesingle = ivec2(0, 135274496);
const ivec2 char_arrowleft = ivec2(1090519040, 1053168);
const ivec2 char_onehalf = ivec2(-1543240648, 33818641);
const ivec2 char_arrowright = ivec2(16777216, 1065457);
const ivec2 char_arrowup = ivec2(-2130706432, 1077584);
const ivec2 char_emdash = ivec2(0, 496);
const ivec2 char_onequarter = ivec2(1175067680, 67637282);
const ivec2 char_dblverticalbar = ivec2(1116012544, 338186401);
const ivec2 char_threequarters = ivec2(1711938592, 101718082);
const ivec2 char_divide = ivec2(16777216, 1049072);
const ivec2 char_comma_saa5051 = ivec2(8454400, 0);
const ivec2 char_period_saa5051 = ivec2(25362432, 0);
const ivec2 char_colon_saa5051 = ivec2(65536, 32);
const ivec2 char_semicolon_saa5051 = ivec2(8454400, 4096);
const ivec2 char_section = ivec2(596119836, 474220770);
const ivec2 char_Adieresis = ivec2(-465010688, 335573267);
const ivec2 char_Odieresis = ivec2(608632832, 335573266);
const ivec2 char_Udieresis = ivec2(608632832, 335579410);
const ivec2 char_degree = ivec2(0, 407396352);
const ivec2 char_adieresis = ivec2(-1001455616, 335573251);
const ivec2 char_odieresis = ivec2(608632832, 2621666);
const ivec2 char_udieresis = ivec2(609157120, 2621714);
const ivec2 char_germandbls = ivec2(608600322, 203704530);
const ivec2 char_currency = ivec2(1133019136, 35041);
const ivec2 char_Eacute = ivec2(-531660800, 269547537);
const ivec2 char_D_saa5052 = ivec2(1149698048, 474517794);
const ivec2 char_L_saa5052 = ivec2(1083113472, 67637280);
const ivec2 char_Aring = ivec2(-465010688, 134246675);
const ivec2 char_eacute = ivec2(-532217856, 269512979);
const ivec2 char_aring = ivec2(-1001455616, 134246659);
const ivec2 char_ccedilla = ivec2(542050312, 61456);
const ivec2 char_ugrave = ivec2(609157120, 68192530);
const ivec2 char_agrave = ivec2(-1001455616, 68186371);
const ivec2 char_ograve = ivec2(608632832, 68157666);
const ivec2 char_egrave = ivec2(-532217856, 68186387);
const ivec2 char_igrave = ivec2(-2130247680, 68157536);
const ivec2 char_idieresis = ivec2(-2130247680, 335556672);
const ivec2 char_edieresis = ivec2(-532217856, 335573267);
const ivec2 char_ecircumflex = ivec2(-532217856, 136868115);
const ivec2 char_ugrave_saa5054 = ivec2(609157120, 136349970);
const ivec2 char_icircumflex = ivec2(-2130247680, 136839264);
const ivec2 char_acircumflex = ivec2(-1001455616, 136868099);
const ivec2 char_ocircumflex_saa5054 = ivec2(608632832, 136868114);
const ivec2 char_ucircumflex = ivec2(609157120, 136839442);
const ivec2 char_ccedilla_saa5054 = ivec2(542050328, 61456);




ivec2 text_box_chars = ivec2(16, 5);


float draw_char(vec2 coord, ivec2 char) {
    int pos_id = int(coord.x * 7.0) + int(coord.y * 9.0) * 7;
    int num = pos_id < 32 ? char.x : char.y;
    
    int val = (1 << pos_id) & num;
    float col = val == 0 ? 0.0 : 1.0;
    
    return col;
}

float draw_text(vec2 coord, ivec2[80] text_string, ivec2 text_box) {
    
    int i = 0;
    float out_col = 0.0;
    
    coord.x *= float(text_box.x);
    coord.y *= float(text_box.y);
    int char = int(coord.x) + (text_box.y - int(coord.y) - 1) * text_box.x;
    coord.x -= float(int(coord.x));
    coord.y -= float(int(coord.y));
	return draw_char(coord, text_string[char]);
}

ivec2 int_to_char(int i) {
        ivec2 lookup[10] = ivec2[](
                char_zero,
                char_one,
                char_two,
                char_three,
                char_four,
                char_five,
                char_six,
                char_seven,
                char_eight,
                char_nine                
        );
        return lookup[i];
}

void main() {
        
        ivec2 text_string[80] = ivec2[](
            char_F,
            char_r,
            char_a,
            char_m,
            char_e,
            char_colon,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            
            char_T,
            char_i,
            char_m,
            char_e,
            char_colon,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            
            char_R,
            char_e,
            char_s,
            char_colon,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            
            char_D,
            char_e,
            char_l,
            char_t,
            char_a,
            char_colon,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            
            char_D,
            char_a,
            char_t,
            char_e,
            char_colon,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space,
            char_space
        );
        
        text_string[7] = int_to_char(int(iFrame)/1000 % 10);
        text_string[8] = int_to_char(int(iFrame)/100 % 10);
        text_string[9] = int_to_char(int(iFrame)/10 % 10);
        text_string[10] = int_to_char(int(iFrame) % 10);
        
        float time = float(iTime);
        text_string[16+6] = int_to_char(int(time)/100 % 10);
        text_string[16+7] = int_to_char(int(time)/10 % 10);
        text_string[16+8] = int_to_char(int(time) % 10);
        text_string[16+9] = char_period;
        text_string[16+10] = int_to_char(int(time*10.0) % 10);
        
        text_string[32+5] = int_to_char(int(iResolution.x)/1000%10);
        text_string[32+6] = int_to_char(int(iResolution.x)/100%10);
        text_string[32+7] = int_to_char(int(iResolution.x)/10%10);
        text_string[32+8] = int_to_char(int(iResolution.x)%10);
        
        text_string[32+10] = int_to_char(int(iResolution.y)/1000%10);
        text_string[32+11] = int_to_char(int(iResolution.y)/100%10);
        text_string[32+12] = int_to_char(int(iResolution.y)/10%10);
        text_string[32+13] = int_to_char(int(iResolution.y)%10);
        
        text_string[48+7] = int_to_char(int(iTimeDelta) % 10);
        text_string[48+8] = char_period;
        text_string[48+9] = int_to_char(int(iTimeDelta*10.0) % 10);
        text_string[48+10] = int_to_char(int(iTimeDelta*100.0) % 10);
        text_string[48+11] = int_to_char(int(iTimeDelta*1000.0) % 10);
        
        text_string[64+7] = int_to_char(int(iDate.x)/10 % 10);
        text_string[64+8] = int_to_char(int(iDate.x) % 10);
        text_string[64+9] = char_slash;
        text_string[64+10] = int_to_char(int(iDate.y)/10 % 10);
        text_string[64+11] = int_to_char(int(iDate.y) % 10);
        text_string[64+12] = char_slash;
        text_string[64+13] = int_to_char(int(iDate.z)/1000 % 10);
        text_string[64+14] = int_to_char(int(iDate.z)/100 % 10);
        text_string[64+13] = int_to_char(int(iDate.z)/10 % 10);
        text_string[64+14] = int_to_char(int(iDate.z) % 10);
        
        
        col = vec4(draw_text(FragCoordUV, text_string, text_box_chars));
}
