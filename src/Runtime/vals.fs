namespace AtomicLang
module Vals =
  type ValType =
    | Num
    | Null
    | Error

  type RuntimeVal =
    abstract member Type : ValType with get
  [<StructuredFormatDisplay("got num => {value}")>]
  type NumVal<'a>(value : 'a) =
    interface RuntimeVal with
      member val Type = ValType.Num with get
    end
    member val value : 'a = value with get, set

  type NullVal() =
    interface RuntimeVal with
      member val Type = ValType.Null with get
    end
    member val value = null with get, set

