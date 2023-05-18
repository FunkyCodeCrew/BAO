# Bao Syntax

## TODO LIST

- IR Header info
- Conventional names for common tasks
- general purpose instructions
- SIMD?

## Syntax

### Table of Contents

- #### [IR Header](#ir-header)

- #### [Types](#types)

- ##### [Basic](#basic)

- ##### [Array](#array)

- ##### [Pointer](#pointer)

- #### [Functions](#function-attributes)

- ##### [Attributes](#attributes)

## IR Header

```ruby
pragma v0.1.0
```

## Types

```ruby
$type Basic NAME { alignment: u32, size: u32 }
$type Array NAME $ty_alias u64
$type Pointer $ty_alias
```

```ruby
$type Basic u8 { alignment: 1, size: 1 }
$type Pointer $u8
```

### Basic

### Array

### Pointer

## Functions

```ruby
@attribute inline always
@attribute overload i32_add
%fun add $i32_ty $i32_ty : $i32_ty
  load p0
  load p1
  i32_add
  return
end%
```

### Function Attributes
