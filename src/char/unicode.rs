//! Unicode character predicates.

use crate::char::macros::predicate;

predicate! {
    Name = Ascii,
    Check = is_ascii,
    Doc = "Checks whether the given character is within the ASCII range.",
    Expected = "ascii character",
    Code = char::ascii,
    Error = "Non-ASCII character encountered.",
    Message = "non-ascii character",
    Help = "make sure the character is within the ASCII range",
}

predicate! {
    Name = Alphabetic,
    Check = is_alphabetic,
    Doc = "Checks whether the given character is alphabetic.",
    Expected = "alphabetic character",
    Code = char::alphabetic,
    Error = "Non-alphabetic character encountered.",
    Message = "non-alphabetic character",
    Help = "make sure the character is alphabetic",
}

predicate! {
    Name = Alphanumeric,
    Check = is_alphanumeric,
    Doc = "Checks whether the given character is alphanumeric.",
    Expected = "alphanumeric character",
    Code = char::alphanumeric,
    Error = "Non-alphanumeric character encountered.",
    Message = "non-alphanumeric character",
    Help = "make sure the character is alphanumeric",
}

predicate! {
    Name = Control,
    Check = is_control,
    Doc = "Checks whether the given character is control.",
    Expected = "control character",
    Code = char::control,
    Error = "Non-control character encountered.",
    Message = "non-control character",
    Help = "make sure the character is control",
}

predicate! {
    Name = Numeric,
    Check = is_numeric,
    Doc = "Checks whether the given character is numeric.",
    Expected = "numeric character",
    Code = char::numeric,
    Error = "Non-numeric character encountered.",
    Message = "non-numeric character",
    Help = "make sure the character is numeric",
}

predicate! {
    Name = Lowercase,
    Check = is_lowercase,
    Doc = "Checks whether the given character is lowercase.",
    Expected = "lowercase character",
    Code = char::lowercase,
    Error = "Non-lowercase character encountered.",
    Message = "non-lowercase character",
    Help = "make sure the character is lowercase",
}

predicate! {
    Name = Uppercase,
    Check = is_uppercase,
    Doc = "Checks whether the given character is uppercase.",
    Expected = "uppercase character",
    Code = char::uppercase,
    Error = "Non-uppercase character encountered.",
    Message = "non-uppercase character",
    Help = "make sure the character is uppercase",
}

predicate! {
    Name = Whitespace,
    Check = is_whitespace,
    Doc = "Checks whether the given character is whitespace.",
    Expected = "whitespace character",
    Code = char::whitespace,
    Error = "Non-whitespace character encountered.",
    Message = "non-whitespace character",
    Help = "make sure the character is whitespace",
}
