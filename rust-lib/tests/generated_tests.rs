/**
 * This is a generated file by build.rs.
 * Do not edit manually.
 */
macro_rules! generate_test_positive_parsing_test {
  ($($case:ident: ($local_part:literal, $domain:literal),)+) => {
    #[cfg(test)]
    mod parses_valid_email_address {
      use email_address_parser::email_address::EmailAddress;
      use wasm_bindgen_test::*;
      $(
        #[test]
        #[wasm_bindgen_test]
        fn $case() {
          let address_str = concat!($local_part, "@", $domain);
          let address = EmailAddress::parse(&address_str, Some(true));
          assert_eq!(address.is_some(), true, "expected {} to be parsed", address_str);
          let address = address.unwrap();
          assert_eq!(address.get_local_part(), $local_part, "local_part of {}", address_str);
          assert_eq!(address.get_domain(), $domain, "domain of {}", address_str);
          assert_eq!(format!("{}", address), address_str, "incorrect display");
        }
      )*
    }
  };
}

generate_test_positive_parsing_test!{
  case1: ("test", "google.com"),
  case2: ("test", "m.maselkowski.pl"),
  case3: ("test", "xn--masekowski-d0b.pl"),
  case4: ("test", "[127.0.0.0]"),
  case5: ("test", "[192.168.0.1]"),
  case6: ("test", "[1.2.3.4]"),
  case7: ("test", "[0.0.0.1]"),
  case8: ("test", "[255.255.255.254]"),
  case9: ("a", "google.com"),
  case10: ("a", "m.maselkowski.pl"),
  case11: ("a", "xn--masekowski-d0b.pl"),
  case12: ("a", "[127.0.0.0]"),
  case13: ("a", "[192.168.0.1]"),
  case14: ("a", "[1.2.3.4]"),
  case15: ("a", "[0.0.0.1]"),
  case16: ("a", "[255.255.255.254]"),
  case17: ("!#$%&'*+-/=?^_`{|}~", "google.com"),
  case18: ("!#$%&'*+-/=?^_`{|}~", "m.maselkowski.pl"),
  case19: ("!#$%&'*+-/=?^_`{|}~", "xn--masekowski-d0b.pl"),
  case20: ("!#$%&'*+-/=?^_`{|}~", "[127.0.0.0]"),
  case21: ("!#$%&'*+-/=?^_`{|}~", "[192.168.0.1]"),
  case22: ("!#$%&'*+-/=?^_`{|}~", "[1.2.3.4]"),
  case23: ("!#$%&'*+-/=?^_`{|}~", "[0.0.0.1]"),
  case24: ("!#$%&'*+-/=?^_`{|}~", "[255.255.255.254]"),
  case25: ("\"test test\"", "google.com"),
  case26: ("\"test test\"", "m.maselkowski.pl"),
  case27: ("\"test test\"", "xn--masekowski-d0b.pl"),
  case28: ("\"test test\"", "[127.0.0.0]"),
  case29: ("\"test test\"", "[192.168.0.1]"),
  case30: ("\"test test\"", "[1.2.3.4]"),
  case31: ("\"test test\"", "[0.0.0.1]"),
  case32: ("\"test test\"", "[255.255.255.254]"),
  case33: ("\"test\\ test\"", "google.com"),
  case34: ("\"test\\ test\"", "m.maselkowski.pl"),
  case35: ("\"test\\ test\"", "xn--masekowski-d0b.pl"),
  case36: ("\"test\\ test\"", "[127.0.0.0]"),
  case37: ("\"test\\ test\"", "[192.168.0.1]"),
  case38: ("\"test\\ test\"", "[1.2.3.4]"),
  case39: ("\"test\\ test\"", "[0.0.0.1]"),
  case40: ("\"test\\ test\"", "[255.255.255.254]"),
  case41: ("te.st", "google.com"),
  case42: ("te.st", "m.maselkowski.pl"),
  case43: ("te.st", "xn--masekowski-d0b.pl"),
  case44: ("te.st", "[127.0.0.0]"),
  case45: ("te.st", "[192.168.0.1]"),
  case46: ("te.st", "[1.2.3.4]"),
  case47: ("te.st", "[0.0.0.1]"),
  case48: ("te.st", "[255.255.255.254]"),
  case49: ("\"te\\,st\"", "google.com"),
  case50: ("\"te\\,st\"", "m.maselkowski.pl"),
  case51: ("\"te\\,st\"", "xn--masekowski-d0b.pl"),
  case52: ("\"te\\,st\"", "[127.0.0.0]"),
  case53: ("\"te\\,st\"", "[192.168.0.1]"),
  case54: ("\"te\\,st\"", "[1.2.3.4]"),
  case55: ("\"te\\,st\"", "[0.0.0.1]"),
  case56: ("\"te\\,st\"", "[255.255.255.254]"),
  case57: ("\"te\\;st\"", "google.com"),
  case58: ("\"te\\;st\"", "m.maselkowski.pl"),
  case59: ("\"te\\;st\"", "xn--masekowski-d0b.pl"),
  case60: ("\"te\\;st\"", "[127.0.0.0]"),
  case61: ("\"te\\;st\"", "[192.168.0.1]"),
  case62: ("\"te\\;st\"", "[1.2.3.4]"),
  case63: ("\"te\\;st\"", "[0.0.0.1]"),
  case64: ("\"te\\;st\"", "[255.255.255.254]"),
}

macro_rules! generate_test_negative_parsing_test {
  ($($case:ident: ($local_part:literal, $domain:literal),)+) => {
    #[cfg(test)]
    mod does_not_parse_invalid_email_address {
      use email_address_parser::email_address::EmailAddress;
      use wasm_bindgen_test::*;
      $(
        #[test]
        #[wasm_bindgen_test]
        fn $case() {
          let address_str = concat!($local_part, "@", $domain);
          assert_eq!(EmailAddress::parse(&address_str, Some(true)).is_none(), true, "expected {} not to be parsed", address_str);
        }
      )*
    }
  };
}

generate_test_negative_parsing_test!{
  case1: ("", "google.com"),
  case2: ("", "m.maselkowski.pl"),
  case3: ("", "xn--masekowski-d0b.pl"),
  case4: ("", "[127.0.0.0]"),
  case5: ("", "[192.168.0.1]"),
  case6: ("", "[1.2.3.4]"),
  case7: ("", "[0.0.0.1]"),
  case8: ("", "[255.255.255.254]"),
  case9: ("a\\ud83c", "google.com"),
  case10: ("a\\ud83c", "m.maselkowski.pl"),
  case11: ("a\\ud83c", "xn--masekowski-d0b.pl"),
  case12: ("a\\ud83c", "[127.0.0.0]"),
  case13: ("a\\ud83c", "[192.168.0.1]"),
  case14: ("a\\ud83c", "[1.2.3.4]"),
  case15: ("a\\ud83c", "[0.0.0.1]"),
  case16: ("a\\ud83c", "[255.255.255.254]"),
  case17: ("\"test", "google.com"),
  case18: ("\"test", "m.maselkowski.pl"),
  case19: ("\"test", "xn--masekowski-d0b.pl"),
  case20: ("\"test", "[127.0.0.0]"),
  case21: ("\"test", "[192.168.0.1]"),
  case22: ("\"test", "[1.2.3.4]"),
  case23: ("\"test", "[0.0.0.1]"),
  case24: ("\"test", "[255.255.255.254]"),
  case25: ("test.", "google.com"),
  case26: ("test.", "m.maselkowski.pl"),
  case27: ("test.", "xn--masekowski-d0b.pl"),
  case28: ("test.", "[127.0.0.0]"),
  case29: ("test.", "[192.168.0.1]"),
  case30: ("test.", "[1.2.3.4]"),
  case31: ("test.", "[0.0.0.1]"),
  case32: ("test.", "[255.255.255.254]"),
  case33: ("te..st", "google.com"),
  case34: ("te..st", "m.maselkowski.pl"),
  case35: ("te..st", "xn--masekowski-d0b.pl"),
  case36: ("te..st", "[127.0.0.0]"),
  case37: ("te..st", "[192.168.0.1]"),
  case38: ("te..st", "[1.2.3.4]"),
  case39: ("te..st", "[0.0.0.1]"),
  case40: ("te..st", "[255.255.255.254]"),
  case41: ("<test", "google.com"),
  case42: ("<test", "m.maselkowski.pl"),
  case43: ("<test", "xn--masekowski-d0b.pl"),
  case44: ("<test", "[127.0.0.0]"),
  case45: ("<test", "[192.168.0.1]"),
  case46: ("<test", "[1.2.3.4]"),
  case47: ("<test", "[0.0.0.1]"),
  case48: ("<test", "[255.255.255.254]"),
  case49: ("test>", "google.com"),
  case50: ("test>", "m.maselkowski.pl"),
  case51: ("test>", "xn--masekowski-d0b.pl"),
  case52: ("test>", "[127.0.0.0]"),
  case53: ("test>", "[192.168.0.1]"),
  case54: ("test>", "[1.2.3.4]"),
  case55: ("test>", "[0.0.0.1]"),
  case56: ("test>", "[255.255.255.254]"),
  case57: (".test", "google.com"),
  case58: (".test", "m.maselkowski.pl"),
  case59: (".test", "xn--masekowski-d0b.pl"),
  case60: (".test", "[127.0.0.0]"),
  case61: (".test", "[192.168.0.1]"),
  case62: (".test", "[1.2.3.4]"),
  case63: (".test", "[0.0.0.1]"),
  case64: (".test", "[255.255.255.254]"),
  case65: ("\"test\\ t\"est\"", "google.com"),
  case66: ("\"test\\ t\"est\"", "m.maselkowski.pl"),
  case67: ("\"test\\ t\"est\"", "xn--masekowski-d0b.pl"),
  case68: ("\"test\\ t\"est\"", "[127.0.0.0]"),
  case69: ("\"test\\ t\"est\"", "[192.168.0.1]"),
  case70: ("\"test\\ t\"est\"", "[1.2.3.4]"),
  case71: ("\"test\\ t\"est\"", "[0.0.0.1]"),
  case72: ("\"test\\ t\"est\"", "[255.255.255.254]"),
  case73: ("\"test t\"est\"", "google.com"),
  case74: ("\"test t\"est\"", "m.maselkowski.pl"),
  case75: ("\"test t\"est\"", "xn--masekowski-d0b.pl"),
  case76: ("\"test t\"est\"", "[127.0.0.0]"),
  case77: ("\"test t\"est\"", "[192.168.0.1]"),
  case78: ("\"test t\"est\"", "[1.2.3.4]"),
  case79: ("\"test t\"est\"", "[0.0.0.1]"),
  case80: ("\"test t\"est\"", "[255.255.255.254]"),
  case81: ("te\";\"st", "google.com"),
  case82: ("te\";\"st", "m.maselkowski.pl"),
  case83: ("te\";\"st", "xn--masekowski-d0b.pl"),
  case84: ("te\";\"st", "[127.0.0.0]"),
  case85: ("te\";\"st", "[192.168.0.1]"),
  case86: ("te\";\"st", "[1.2.3.4]"),
  case87: ("te\";\"st", "[0.0.0.1]"),
  case88: ("te\";\"st", "[255.255.255.254]"),
  case89: ("te;st", "google.com"),
  case90: ("te;st", "m.maselkowski.pl"),
  case91: ("te;st", "xn--masekowski-d0b.pl"),
  case92: ("te;st", "[127.0.0.0]"),
  case93: ("te;st", "[192.168.0.1]"),
  case94: ("te;st", "[1.2.3.4]"),
  case95: ("te;st", "[0.0.0.1]"),
  case96: ("te;st", "[255.255.255.254]"),
  case97: ("test", ""),
  case98: ("test", "example.com>"),
  case99: ("test", "test..com"),
  case100: ("test", ".test.com"),
  case101: ("test", "test.com."),
  case102: ("test", "google.."),
  case103: ("test", ".com"),
  case104: ("test", "google-.com"),
  case105: ("test", "-google-.com"),
  case106: ("a", ""),
  case107: ("a", "example.com>"),
  case108: ("a", "test..com"),
  case109: ("a", ".test.com"),
  case110: ("a", "test.com."),
  case111: ("a", "google.."),
  case112: ("a", ".com"),
  case113: ("a", "google-.com"),
  case114: ("a", "-google-.com"),
  case115: ("!#$%&'*+-/=?^_`{|}~", ""),
  case116: ("!#$%&'*+-/=?^_`{|}~", "example.com>"),
  case117: ("!#$%&'*+-/=?^_`{|}~", "test..com"),
  case118: ("!#$%&'*+-/=?^_`{|}~", ".test.com"),
  case119: ("!#$%&'*+-/=?^_`{|}~", "test.com."),
  case120: ("!#$%&'*+-/=?^_`{|}~", "google.."),
  case121: ("!#$%&'*+-/=?^_`{|}~", ".com"),
  case122: ("!#$%&'*+-/=?^_`{|}~", "google-.com"),
  case123: ("!#$%&'*+-/=?^_`{|}~", "-google-.com"),
  case124: ("\"test test\"", ""),
  case125: ("\"test test\"", "example.com>"),
  case126: ("\"test test\"", "test..com"),
  case127: ("\"test test\"", ".test.com"),
  case128: ("\"test test\"", "test.com."),
  case129: ("\"test test\"", "google.."),
  case130: ("\"test test\"", ".com"),
  case131: ("\"test test\"", "google-.com"),
  case132: ("\"test test\"", "-google-.com"),
  case133: ("\"test\\ test\"", ""),
  case134: ("\"test\\ test\"", "example.com>"),
  case135: ("\"test\\ test\"", "test..com"),
  case136: ("\"test\\ test\"", ".test.com"),
  case137: ("\"test\\ test\"", "test.com."),
  case138: ("\"test\\ test\"", "google.."),
  case139: ("\"test\\ test\"", ".com"),
  case140: ("\"test\\ test\"", "google-.com"),
  case141: ("\"test\\ test\"", "-google-.com"),
  case142: ("te.st", ""),
  case143: ("te.st", "example.com>"),
  case144: ("te.st", "test..com"),
  case145: ("te.st", ".test.com"),
  case146: ("te.st", "test.com."),
  case147: ("te.st", "google.."),
  case148: ("te.st", ".com"),
  case149: ("te.st", "google-.com"),
  case150: ("te.st", "-google-.com"),
  case151: ("\"te\\,st\"", ""),
  case152: ("\"te\\,st\"", "example.com>"),
  case153: ("\"te\\,st\"", "test..com"),
  case154: ("\"te\\,st\"", ".test.com"),
  case155: ("\"te\\,st\"", "test.com."),
  case156: ("\"te\\,st\"", "google.."),
  case157: ("\"te\\,st\"", ".com"),
  case158: ("\"te\\,st\"", "google-.com"),
  case159: ("\"te\\,st\"", "-google-.com"),
  case160: ("\"te\\;st\"", ""),
  case161: ("\"te\\;st\"", "example.com>"),
  case162: ("\"te\\;st\"", "test..com"),
  case163: ("\"te\\;st\"", ".test.com"),
  case164: ("\"te\\;st\"", "test.com."),
  case165: ("\"te\\;st\"", "google.."),
  case166: ("\"te\\;st\"", ".com"),
  case167: ("\"te\\;st\"", "google-.com"),
  case168: ("\"te\\;st\"", "-google-.com"),
  case169: ("", ""),
  case170: ("", "example.com>"),
  case171: ("", "test..com"),
  case172: ("", ".test.com"),
  case173: ("", "test.com."),
  case174: ("", "google.."),
  case175: ("", ".com"),
  case176: ("", "google-.com"),
  case177: ("", "-google-.com"),
  case178: ("a\\ud83c", ""),
  case179: ("a\\ud83c", "example.com>"),
  case180: ("a\\ud83c", "test..com"),
  case181: ("a\\ud83c", ".test.com"),
  case182: ("a\\ud83c", "test.com."),
  case183: ("a\\ud83c", "google.."),
  case184: ("a\\ud83c", ".com"),
  case185: ("a\\ud83c", "google-.com"),
  case186: ("a\\ud83c", "-google-.com"),
  case187: ("\"test", ""),
  case188: ("\"test", "example.com>"),
  case189: ("\"test", "test..com"),
  case190: ("\"test", ".test.com"),
  case191: ("\"test", "test.com."),
  case192: ("\"test", "google.."),
  case193: ("\"test", ".com"),
  case194: ("\"test", "google-.com"),
  case195: ("\"test", "-google-.com"),
  case196: ("test.", ""),
  case197: ("test.", "example.com>"),
  case198: ("test.", "test..com"),
  case199: ("test.", ".test.com"),
  case200: ("test.", "test.com."),
  case201: ("test.", "google.."),
  case202: ("test.", ".com"),
  case203: ("test.", "google-.com"),
  case204: ("test.", "-google-.com"),
  case205: ("te..st", ""),
  case206: ("te..st", "example.com>"),
  case207: ("te..st", "test..com"),
  case208: ("te..st", ".test.com"),
  case209: ("te..st", "test.com."),
  case210: ("te..st", "google.."),
  case211: ("te..st", ".com"),
  case212: ("te..st", "google-.com"),
  case213: ("te..st", "-google-.com"),
  case214: ("<test", ""),
  case215: ("<test", "example.com>"),
  case216: ("<test", "test..com"),
  case217: ("<test", ".test.com"),
  case218: ("<test", "test.com."),
  case219: ("<test", "google.."),
  case220: ("<test", ".com"),
  case221: ("<test", "google-.com"),
  case222: ("<test", "-google-.com"),
  case223: ("test>", ""),
  case224: ("test>", "example.com>"),
  case225: ("test>", "test..com"),
  case226: ("test>", ".test.com"),
  case227: ("test>", "test.com."),
  case228: ("test>", "google.."),
  case229: ("test>", ".com"),
  case230: ("test>", "google-.com"),
  case231: ("test>", "-google-.com"),
  case232: (".test", ""),
  case233: (".test", "example.com>"),
  case234: (".test", "test..com"),
  case235: (".test", ".test.com"),
  case236: (".test", "test.com."),
  case237: (".test", "google.."),
  case238: (".test", ".com"),
  case239: (".test", "google-.com"),
  case240: (".test", "-google-.com"),
  case241: ("\"test\\ t\"est\"", ""),
  case242: ("\"test\\ t\"est\"", "example.com>"),
  case243: ("\"test\\ t\"est\"", "test..com"),
  case244: ("\"test\\ t\"est\"", ".test.com"),
  case245: ("\"test\\ t\"est\"", "test.com."),
  case246: ("\"test\\ t\"est\"", "google.."),
  case247: ("\"test\\ t\"est\"", ".com"),
  case248: ("\"test\\ t\"est\"", "google-.com"),
  case249: ("\"test\\ t\"est\"", "-google-.com"),
  case250: ("\"test t\"est\"", ""),
  case251: ("\"test t\"est\"", "example.com>"),
  case252: ("\"test t\"est\"", "test..com"),
  case253: ("\"test t\"est\"", ".test.com"),
  case254: ("\"test t\"est\"", "test.com."),
  case255: ("\"test t\"est\"", "google.."),
  case256: ("\"test t\"est\"", ".com"),
  case257: ("\"test t\"est\"", "google-.com"),
  case258: ("\"test t\"est\"", "-google-.com"),
  case259: ("te\";\"st", ""),
  case260: ("te\";\"st", "example.com>"),
  case261: ("te\";\"st", "test..com"),
  case262: ("te\";\"st", ".test.com"),
  case263: ("te\";\"st", "test.com."),
  case264: ("te\";\"st", "google.."),
  case265: ("te\";\"st", ".com"),
  case266: ("te\";\"st", "google-.com"),
  case267: ("te\";\"st", "-google-.com"),
  case268: ("te;st", ""),
  case269: ("te;st", "example.com>"),
  case270: ("te;st", "test..com"),
  case271: ("te;st", ".test.com"),
  case272: ("te;st", "test.com."),
  case273: ("te;st", "google.."),
  case274: ("te;st", ".com"),
  case275: ("te;st", "google-.com"),
  case276: ("te;st", "-google-.com"),
}

macro_rules! generate_is_email_test {
  ($($case:ident: ($email:literal, $is_email:literal),)+) => {
    #[cfg(test)]
    mod is_email_tests {
      use email_address_parser::email_address::EmailAddress;
      use wasm_bindgen_test::*;
      $(
        #[test]
        #[wasm_bindgen_test]
        fn $case() {
          let email = EmailAddress::parse(&$email, None);
          assert_eq!(email.is_some(), $is_email, "expected {} to be valid: {}", $email, $is_email);
          if $is_email {
            assert_eq!(
              format!("{}", email.unwrap()), 
              $email, 
              "incorrect display"
            );
          }
        }
      )*
    }
  };
}

generate_is_email_test!{
  case1: ("", false),
  case2: ("test", false),
  case3: ("@", false),
  case4: ("test@", false),
  case5: ("test@io", true),
  case6: ("@io", false),
  case7: ("@iana.org", false),
  case8: ("test@iana.org", true),
  case9: ("test@nominet.org.uk", true),
  case10: ("test@about.museum", true),
  case11: ("a@iana.org", true),
  case12: ("test@e.com", true),
  case13: ("test@iana.a", true),
  case14: ("test.test@iana.org", true),
  case15: (".test@iana.org", false),
  case16: ("test.@iana.org", false),
  case17: ("test..iana.org", false),
  case18: ("test_exa-mple.com", false),
  case19: ("!#$%&`*+/=?^`{|}~@iana.org", true),
  case20: ("test\\@test@iana.org", false),
  case21: ("123@iana.org", true),
  case22: ("test@123.com", true),
  case23: ("test@iana.123", true),
  case24: ("test@255.255.255.255", true),
  case25: ("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghiklm@iana.org", true),
  case26: ("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghiklmn@iana.org", true),
  case27: ("test@abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghikl.com", true),
  case28: ("test@abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghiklm.com", true),
  case29: ("test@mason-dixon.com", true),
  case30: ("test@-iana.org", false),
  case31: ("test@iana-.com", false),
  case32: ("test@c--n.com", true),
  case33: ("test@iana.co-uk", true),
  case34: ("test@.iana.org", false),
  case35: ("test@iana.org.", false),
  case36: ("test@iana..com", false),
  case37: ("a@a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v", true),
  case38: ("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghiklm@abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghikl.abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghikl.abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghi", true),
  case39: ("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghiklm@abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghikl.abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghikl.abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghij", true),
  case40: ("a@abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghikl.abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghikl.abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghikl.abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefg.hij", true),
  case41: ("a@abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghikl.abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghikl.abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghikl.abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefg.hijk", true),
  case42: ("\"test\"@iana.org", true),
  case43: ("\"\"@iana.org", true),
  case44: ("\"\"\"@iana.org", false),
  case45: ("\"\\a\"@iana.org", true),
  case46: ("\"\\\"\"@iana.org", true),
  case47: ("\"\\\"@iana.org", false),
  case48: ("\"\\\\\"@iana.org", true),
  case49: ("test\"@iana.org", false),
  case50: ("\"test@iana.org", false),
  case51: ("\"test\"test@iana.org", false),
  case52: ("test\"text\"@iana.org", false),
  case53: ("\"test\"\"test\"@iana.org", false),
  case54: ("\"test\".\"test\"@iana.org", true),
  case55: ("\"test\\ test\"@iana.org", true),
  case56: ("\"test\".test@iana.org", true),
  case57: ("\"test\u{00}\"@iana.org", false),
  case58: ("\"test\\\u{00}\"@iana.org", true),
  case59: ("\"abcdefghijklmnopqrstuvwxyz abcdefghijklmnopqrstuvwxyz abcdefghj\"@iana.org", true),
  case60: ("\"abcdefghijklmnopqrstuvwxyz abcdefghijklmnopqrstuvwxyz abcdefg\\h\"@iana.org", true),
  case61: ("test@[255.255.255.255]", true),
  case62: ("test@a[255.255.255.255]", false),
  case63: ("test@[255.255.255]", true),
  case64: ("test@[255.255.255.255.255]", true),
  case65: ("test@[255.255.255.256]", true),
  case66: ("test@[1111:2222:3333:4444:5555:6666:7777:8888]", true),
  case67: ("test@[IPv6:1111:2222:3333:4444:5555:6666:7777]", true),
  case68: ("test@[IPv6:1111:2222:3333:4444:5555:6666:7777:8888]", true),
  case69: ("test@[IPv6:1111:2222:3333:4444:5555:6666:7777:8888:9999]", true),
  case70: ("test@[IPv6:1111:2222:3333:4444:5555:6666:7777:888G]", true),
  case71: ("test@[IPv6:1111:2222:3333:4444:5555:6666::8888]", true),
  case72: ("test@[IPv6:1111:2222:3333:4444:5555::8888]", true),
  case73: ("test@[IPv6:1111:2222:3333:4444:5555:6666::7777:8888]", true),
  case74: ("test@[IPv6::3333:4444:5555:6666:7777:8888]", true),
  case75: ("test@[IPv6:::3333:4444:5555:6666:7777:8888]", true),
  case76: ("test@[IPv6:1111::4444:5555::8888]", true),
  case77: ("test@[IPv6:::]", true),
  case78: ("test@[IPv6:1111:2222:3333:4444:5555:255.255.255.255]", true),
  case79: ("test@[IPv6:1111:2222:3333:4444:5555:6666:255.255.255.255]", true),
  case80: ("test@[IPv6:1111:2222:3333:4444:5555:6666:7777:255.255.255.255]", true),
  case81: ("test@[IPv6:1111:2222:3333:4444::255.255.255.255]", true),
  case82: ("test@[IPv6:1111:2222:3333:4444:5555:6666::255.255.255.255]", true),
  case83: ("test@[IPv6:1111:2222:3333:4444:::255.255.255.255]", true),
  case84: ("test@[IPv6::255.255.255.255]", true),
  case85: (" test @iana.org", true),
  case86: ("test@ iana .com", true),
  case87: ("test . test@iana.org", true),
  case88: ("\u{0d}\u{0a} test@iana.org", true),
  case89: ("\u{0d}\u{0a} \u{0d}\u{0a} test@iana.org", true),
  case90: ("(comment)test@iana.org", true),
  case91: ("((comment)test@iana.org", false),
  case92: ("(comment(comment))test@iana.org", true),
  case93: ("test@(comment)iana.org", true),
  case94: ("test(comment)test@iana.org", false),
  case95: ("test@(comment)[255.255.255.255]", true),
  case96: ("(comment)abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghiklm@iana.org", true),
  case97: ("test@(comment)abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghikl.com", true),
  case98: ("(comment)test@abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghik.abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghik.abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijk.abcdefghijklmnopqrstuvwxyzabcdefghijk.abcdefghijklmnopqrstu", true),
  case99: ("test@iana.org\u{0a}", false),
  case100: ("test@xn--hxajbheg2az3al.xn--jxalpdlp", true),
  case101: ("xn--test@iana.org", true),
  case102: ("test@iana.org-", false),
  case103: ("\"test@iana.org", false),
  case104: ("(test@iana.org", false),
  case105: ("test@(iana.org", false),
  case106: ("test@[1.2.3.4", false),
  case107: ("\"test\\\"@iana.org", false),
  case108: ("(comment\\)test@iana.org", false),
  case109: ("test@iana.org(comment\\)", false),
  case110: ("test@iana.org(comment\\", false),
  case111: ("test@[RFC-5322-domain-literal]", true),
  case112: ("test@[RFC-5322]-domain-literal]", false),
  case113: ("test@[RFC-5322-[domain-literal]", false),
  case117: ("test@[RFC-5322-domain-literal\\]", false),
  case118: ("test@[RFC-5322-domain-literal\\", false),
  case119: ("test@[RFC 5322 domain literal]", true),
  case120: ("test@[RFC-5322-domain-literal] (comment)", true),
  case121: ("@iana.org", false),
  case122: ("test@.org", false),
  case123: ("\"\"@iana.org", true),
  case124: ("\"\\\"@iana.org", true),
  case125: ("()test@iana.org", true),
  case126: ("test@iana.org\u{0d}", false),
  case127: ("\u{0d}test@iana.org", false),
  case128: ("\"\u{0d}test\"@iana.org", false),
  case129: ("(\u{0d})test@iana.org", false),
  case130: ("test@iana.org(\u{0d})", false),
  case131: ("\u{0a}test@iana.org", false),
  case132: ("\"\u{0a}\"@iana.org", false),
  case133: ("\"\\\u{0a}\"@iana.org", true),
  case134: ("(\u{0a})test@iana.org", false),
  case135: ("\u{07}@iana.org", false),
  case136: ("test@\u{07}.org", false),
  case137: ("\"\u{07}\"@iana.org", true),
  case138: ("\"\\\u{07}\"@iana.org", true),
  case139: ("(\u{07})test@iana.org", true),
  case140: ("\u{0d}\u{0a}test@iana.org", false),
  case141: ("\u{0d}\u{0a} \u{0d}\u{0a}test@iana.org", false),
  case142: (" \u{0d}\u{0a}test@iana.org", false),
  case143: (" \u{0d}\u{0a} test@iana.org", true),
  case144: (" \u{0d}\u{0a} \u{0d}\u{0a}test@iana.org", false),
  case145: (" \u{0d}\u{0a}\u{0d}\u{0a}test@iana.org", false),
  case146: (" \u{0d}\u{0a}\u{0d}\u{0a} test@iana.org", false),
  case147: ("test@iana.org\u{0d}\u{0a} ", true),
  case148: ("test@iana.org\u{0d}\u{0a} \u{0d}\u{0a} ", true),
  case149: ("test@iana.org\u{0d}\u{0a}", false),
  case150: ("test@iana.org\u{0d}\u{0a} \u{0d}\u{0a}", false),
  case151: ("test@iana.org \u{0d}\u{0a}", false),
  case152: ("test@iana.org \u{0d}\u{0a} ", true),
  case153: ("test@iana.org \u{0d}\u{0a} \u{0d}\u{0a}", false),
  case154: ("test@iana.org \u{0d}\u{0a}\u{0d}\u{0a}", false),
  case155: ("test@iana.org \u{0d}\u{0a}\u{0d}\u{0a} ", false),
  case156: (" test@iana.org", true),
  case157: ("test@iana.org ", true),
  case158: ("test@[IPv6:1::2:]", true),
  case159: ("\"test\\©\"@iana.org", false),
  case160: ("test@iana/icann.org", true),
  case161: ("test.(comment)test@iana.org", true),
  case162: ("test@org", true),
  case163: ("test@test.com", true),
  case164: ("test@nic.no", true),
}