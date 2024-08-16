{
"public" => Token::Modifier(Modifier::Public),
"private" => Token::Modifier(Modifier::Private),
"protected" => Token::Modifier(Modifier::Protected),
"static" => Token::Modifier(Modifier::Static),
"async" => Token::Modifier(Modifier::Async),

                    "const" => Token::ReservedWord(ReservedWord::Const),
                    "let" => Token::ReservedWord(ReservedWord::Let),
                    "operator" => Token::ReservedWord(ReservedWord::Operator),
                    "constructor" => Token::ReservedWord(ReservedWord::Constructor),
                    "class" => Token::ReservedWord(ReservedWord::Class),
                    "interface" => Token::ReservedWord(ReservedWord::Interface),
                    "implements" => Token::ReservedWord(ReservedWord::Implements),
                    "this" => Token::ReservedWord(ReservedWord::This),
                    "return" => Token::ReservedWord(ReservedWord::Return),
                    "function" => Token::ReservedWord(ReservedWord::Function),
                    "if" => Token::ReservedWord(ReservedWord::If),
                    "else" => Token::ReservedWord(ReservedWord::Else),
                    "new" => Token::ReservedWord(ReservedWord::New),
                    "null" => Token::ReservedWord(ReservedWord::Null),
                    "enum" => Token::ReservedWord(ReservedWord::Enum),
                    "namespace" => Token::ReservedWord(ReservedWord::Namespace),
                    "declare" => Token::ReservedWord(ReservedWord::Declare),
                    "export" => Token::ReservedWord(ReservedWord::Export),
                    "import" => Token::ReservedWord(ReservedWord::Import),
                    "default" => Token::ReservedWord(ReservedWord::Default),
                    "when" => Token::ReservedWord(ReservedWord::When),
                    "match" => Token::ReservedWord(ReservedWord::Match),
                    "extends" => Token::ReservedWord(ReservedWord::Extends),
                    "get" => Token::ReservedWord(ReservedWord::Get),
                    "set" => Token::ReservedWord(ReservedWord::Set),
                    "type" => Token::ReservedWord(ReservedWord::Type),
                    "typeOf" => Token::ReservedWord(ReservedWord::TypeOf),
                    "for" => Token::ReservedWord(ReservedWord::For),
                    "in" => Token::ReservedWord(ReservedWord::In),
                    "of" => Token::ReservedWord(ReservedWord::Of),
                    "as" => Token::ReservedWord(ReservedWord::As),
                    "from" => Token::ReservedWord(ReservedWord::From),

                    "any" => Token::BuiltInType(BuiltInType::Any),
                    "number" => Token::BuiltInType(BuiltInType::Number),
                    "float" => Token::BuiltInType(BuiltInType::Float),
                    "boolean" => Token::BuiltInType(BuiltInType::Boolean),
                    "string" => Token::BuiltInType(BuiltInType::String),
                    "symbol" => Token::BuiltInType(BuiltInType::Symbol),
                    "void" => Token::BuiltInType(BuiltInType::Void),

                    "true" => Token::Literal(Literal::Boolean(true)),
                    "false" => Token::Literal(Literal::Boolean(false)),
comma_punctuation: "," => Token::Punctuation(Punctuation::Comma);
dot_punctuation: "." => Token::Punctuation(Punctuation::Dot);
ellipsis_punctuation: "..." => Token::Punctuation(Punctuation::Ellipsis);
fat_arrow_punctuation: "=>" => Token::Punctuation(Punctuation::FatArrow);
double_slash_punctuation: "//" => Token::Punctuation(Punctuation::DoubleSlash);
double_quote_punctuation: "\"" => Token::Punctuation(Punctuation::DoubleQuote);
colon_punctuation: ":" => Token::Punctuation(Punctuation::Colon);
semi_punctuation: ";" => Token::Punctuation(Punctuation::Semi);
question_punctuation: "?" => Token::Punctuation(Punctuation::Question);
pound_punctuation: "#" => Token::Punctuation(Punctuation::Pound);




and_operator: "&" => Token::Operator(Operator::And);
and_and_operator: "&&" => Token::Operator(Operator::AndAnd);
equal_operator: "==" => Token::Operator(Operator::EqEq);
not_equal_operator: "!=" => Token::Operator(Operator::Ne);
or_operator: "|" => Token::Operator(Operator::Or);
or_or_operator: "||" => Token::Operator(Operator::OrOr);
assign_operator: "=" => Token::Operator(Operator::Eq);
plus_plus_operator: "++" => Token::Operator(Operator::PlusPlus);
plus_operator: "+" => Token::Operator(Operator::Plus);
minus_minus_operator: "--" => Token::Operator(Operator::MinusMinus);
minus_operator: "-" => Token::Operator(Operator::Minus);
multiply_operator: "*" => Token::Operator(Operator::Star);
divide_operator: "/" => Token::Operator(Operator::Slash);
not_operator: "!" => Token::Operator(Operator::Not);
greater_equal_operator: ">=" => Token::Operator(Operator::Ge);
lesser_equal_operator: "<=" => Token::Operator(Operator::Le);
greater_operator: ">" => Token::Operator(Operator::Gt);
lesser_operator: "<" => Token::Operator(Operator::Lt);