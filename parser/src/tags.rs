use nom::{
    bytes::complete::take,
    combinator::{map, verify},
    error::Error,
    sequence::tuple,
    Parser,
};
use nom::bytes::complete::tag;
use crate::input::{Input, PineResult, Positioned, Span};
use crate::lexer::token::Token;

macro_rules! tags {
    ($($func_name:ident => $tag:expr;)*) => {
        $(
            pub fn $func_name(input: Input<'_>) -> PineResult<Positioned<Input<'_>>> {
                verify(map(tag($tag), |s: Input<'_>| to_positioned(s)), |pos: &Positioned<Input<'_>>| {
                    println!("{:?}", pos.value.fragment());
                    pos.value.fragment().to_string() == $tag
                })(input)

            }
        )*
    };
}

tags! {
    const_tag => "const";
    let_tag => "let";
    // constructor_tag => Token::ReservedWord(ReservedWord::Constructor);
    // class_tag => Token::ReservedWord(ReservedWord::Class);
    // interface_tag => Token::ReservedWord(ReservedWord::Interface);
    // implements_tag => Token::ReservedWord(ReservedWord::Implements);
     this_tag => "this";
    return_tag => "return";
     function_tag => "function";
    if_tag => "if";
     else_tag =>"else";
    new_tag => "new";
    null_tag => "null";
     enum_tag => "enum";
    // namespace_tag => Token::ReservedWord(ReservedWord::Namespace);
    // decalre_tag => Token::ReservedWord(ReservedWord::Declare);
     export_tag =>"export";
    import_tag => "import";
    // default_tag => Token::ReservedWord(ReservedWord::Default);
    // match_tag => Token::ReservedWord(ReservedWord::Match);
   extends_tag => "extends";
    // get_tag => Token::ReservedWord(ReservedWord::Get);
    // set_tag => Token::ReservedWord(ReservedWord::Set);
    type_tag => "type";
    typeof_tag => "typeOf";
    for_tag => "for";
    // in_tag => Token::ReservedWord(ReservedWord::In);
    // of_tag => Token::ReservedWord(ReservedWord::Of);
    // as_tag => Token::ReservedWord(ReservedWord::As);
     from_tag => "from";
    // when_tag => Token::ReservedWord(ReservedWord::When);
    //
    comma_tag => ",";
    dot_tag =>".";
    ellipsis_tag =>"...";
    fat_arrow_tag =>"=>";
    // double_slash_tag => Token::Punctuation(Punctuation::DoubleSlash);
    // double_quote_tag => Token::Punctuation(Punctuation::DoubleQuote);
     colon_tag => ":";
    semi_tag => ";";
    question_tag => "?";
    // pound_tag => Token::Punctuation(Punctuation::Pound);
    //
    and_tag => "&";
    and_and_tag =>"&&";
    plus_tag => "+";
    // star_tag => Token::Operator(Operator::Star);
    // slash_tag => Token::Operator(Operator::Slash);
    or_tag => "|";
    or_or_tag =>"||";
    plus_plus_tag => "++";
    // minus_tag => Token::Operator(Operator::Minus);
    // minus_minus_tag => Token::Operator(Operator::MinusMinus);
    eq_eq_tag => "==";
    eq_tag =>"=";
     ne_tag => "!=";
     le_tag =>"<=";
     ge_tag => ">=";
     lt_tag =>"<";
     gt_tag => ">";
     not_tag => "!";
    //
    public_tag => "public";
    private_tag => "private";
    protected_tag => "protected";
    // static_tag => Token::Modifier(Modifier::Static);
    async_tag => "async";
    //
     any_tag => "any";
     number_tag => "number";
     float_tag =>"float";


     boolean_tag => "boolean";
     string_tag =>"string";
     symbol_tag => "symbol";
     void_tag => "void";
    //
    brace_open_tag => "{";
    brace_close_tag => "}";
    bracket_open_tag => "[";
    bracket_close_tag =>"]";
    paren_open_tag => "(";
    paren_close_tag => ")";
    //
    //eof_tag => Token::EOF;
}



fn to_positioned(span: Input<'_>) -> Positioned<Input<'_>> {
    let input = span.clone();
    Positioned::new(span, Span::from(input))
}

pub fn ok_tag(input: Input<'_>) -> PineResult<Positioned<Input<'_>>> {

    // 使用 verify 进行验证
    verify(map(tag("ok"), |s: Input<'_>| to_positioned(s)), |pos: &Positioned<Input<'_>>| {
        println!("{:?}", pos.value.fragment());
        pos.value.fragment().to_string() == "ok" // 确保片段值为 "ok"
    })(input)
}