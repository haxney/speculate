use extra::json::ToJson;
use extra::json;
use std::str;

use lexer::*;

pub fn json_almost_equals(a: &json::Json, b: &json::Json) -> bool {
    match (a, b) {
        (&json::Number(a), &json::Number(b)) => (a - b).abs() < 1e-6,
        (&json::String(ref a), &json::String(ref b)) => a == b,
        (&json::Boolean(a), &json::Boolean(b)) => a == b,
        (&json::List(ref a), &json::List(ref b))
            => a.len() == b.len() && a.iter().zip(b.iter()).all(|(ref a, ref b)| json_almost_equals(*a, *b)),
        (&json::Object(_), &json::Object(_)) => fail!(~"Not implemented"),
        (&json::Null, &json::Null) => true,
        _ => false,
    }
}

impl ToJson for Token {
    fn to_json(&self) -> json::Json {
        use JList = extra::json::List;
        use JString = extra::json::String;

        fn numeric(value: &NumericValue) -> ~[json::Json] {
            match *value {
                NumericValue{representation: ref r, value: ref v, int_value: ref i}
                => ~[r.to_json(), v.to_json(),
                     JString(match *i { Some(_) => ~"integer", _ => ~"number" })]
            }
        }

        match *self {
            Ident(ref value) => JList(~[JString(~"ident"), value.to_json()]),
            Function(ref name)
                => JList(~[JString(~"function"), name.to_json()]),
            AtKeyword(ref value)
                => JList(~[JString(~"at-keyword"), value.to_json()]),
            Hash(ref value)
                => JList(~[JString(~"hash"), value.to_json(),
                           JString(~"unrestricted")]),
            IDHash(ref value)
                => JList(~[JString(~"hash"), value.to_json(), JString(~"id")]),
            String(ref value) => JList(~[JString(~"string"), value.to_json()]),
            BadString => JList(~[JString(~"error"), JString(~"bad-string")]),
            URL(ref value) => JList(~[JString(~"url"), value.to_json()]),
            BadURL => JList(~[JString(~"error"), JString(~"bad-url")]),
            Delim('\\') => JString(~"\\"),
            Delim(value) => JString(str::from_char(value)),

            Number(ref value) => JList(~[JString(~"number")] + numeric(value)),
            Percentage(ref value)
                => JList(~[JString(~"percentage")] + numeric(value)),
            Dimension(ref value, ref unit)
                => JList(~[JString(~"dimension")]
                         + numeric(value)
                         + ~[unit.to_json()]),

            UnicodeRange(s, e)
                => JList(~[JString(~"unicode-range"),
                           s.to_json(),
                           e.to_json()]),
            IncludeMatch => JString(~"~="),
            DashMatch => JString(~"|="),
            PrefixMatch => JString(~"^="),
            SuffixMatch => JString(~"$="),
            SubstringMatch => JString(~"*="),
            Column => JString(~"||"),
            WhiteSpace => JString(~" "),

            CDO => JString(~"<!--"),
            CDC => JString(~"-->"),

            Colon => JString(~":"),
            Semicolon => JString(~";"),
            Comma => JString(~","),

            LeftBracket => JString(~"["),
            RightBracket => JString(~"]"),
            LeftParen => JString(~"("),
            RightParen => JString(~")"),
            LeftCurlyBracket => JString(~"{"),
            RightCurlyBracket => JString(~"}"),
        }
    }
}

impl ToJson for SourceLocation {
    fn to_json(&self) -> json::Json {
        use JList = extra::json::List;

        JList(~[self.line.to_json(), self.column.to_json()])
    }
}

pub fn list_to_json(list: &~[(Token, SourceLocation)]) -> ~[json::Json] {
    list.map(|tuple| {
        match *tuple {
            (ref c, _) => c.to_json()
        }
    })
}
