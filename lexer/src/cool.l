%%
[0-9] "DIGIT"
[a-z] "LOWERCASE_LETTER"
[A-Z] "UPPERCASE_LETTER"
[a-zA-Z] "LETTER"
( |\f|\r\|\t|\v) "BLANK"
(\+|\-|\*|/) "OPERATOR"
(~|<|=|\(|\)|\{|\}|;|:|.|,|@) "SINGLE_CHAR_TOKEN"
(?i:class) "CLASS"
(?i:else) "ELSE"
(?i:fi) "FI"
(?i:if) "IF"
(?i:in) "IN"
(?i:inherits) "INHERITS"
(?i:let) "LET"
(?i:loop) "LOOP"
(?i:pool) "POOL"
(?i:then) "THEN"
(?i:while) "WHILE"
(?i:case) "CASE"
(?i:esac) "ESAC"
(?i:of) "OF"
(?i:new) "NEW"
(?i:isvoid) "ISVOID"
(?i:not) "NOT"

[0-9]+ "INT_CONST"