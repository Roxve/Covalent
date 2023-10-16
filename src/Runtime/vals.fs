namespace AtomicLang
module Vals =
  type ValType =
    | Num
    | Error

  type RuntimeVal<'a> =
    abstract member Type : ValType with get
    abstract member value : 'a with get, set
  type NumVal<'a>(value : 'a) =
    interface RuntimeVal<'a> with
      member val Type = ValType.Num with get
      member val value = value with get, set
    end
