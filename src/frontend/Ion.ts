export interface Ion {
  value: string
  type: Type
  line: number
  colmun: number
}
export enum Type {
  //keywords
  set_kw,
  locked_kw,
  use_kw,
  if_kw,
  else_kw,
  
  //types
  str_type,
  num_type,
  null_type,

  //
  ooperator,
  id,
  setter,

  //symbols
  Dot,
  OpenParen,
  CloseParen,
  OpenBrace,
  CloseBrace,
  OpenBracket,
  CloseBracket,
  Colon,
  Comma,

  EOF,
}
