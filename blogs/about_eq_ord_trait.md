# 一文讲透Rust中的PartialEq和Eq
<!-- TOC -->
  * [前言](#前言)
  * [1. 数学中的相等关系](#1-数学中的相等关系)
    * [1.1 部分相等关系](#11-部分相等关系)
    * [1.2 部分相等与全相等的关系](#12-部分相等与全相等的关系)
    * [1.3 小结](#13-小结)
  * [2. 编程与数学的关系](#2-编程与数学的关系)
  * [3. PartialEq](#3-partialeq)
    * [3.1 trait定义](#31-trait定义)
    * [3.2 对应操作符](#32-对应操作符)
    * [3.3 可派生](#33-可派生)
    * [3.4 手动实现PartialEq](#34-手动实现PartialEq)
    * [3.5 比较不同的类型](#35-比较不同的类型)
    * [3.6 Rust基本类型如何实现PartialEq](#36-Rust基本类型如何实现PartialEq)
  * [4. Eq](#4-eq)
    * [4.1 trait定义](#41-trait定义)
    * [4.2 对应操作符](#42-对应操作符)
    * [4.3 可派生](#43-可派生)
    * [4.4 手动实现Eq](#44-手动实现Eq)
    * [4.5 比较不同的类型](#45-比较不同的类型)
    * [4.6 Rust基本类型如何实现Eq](#46-Rust基本类型如何实现Eq)
  * [5. 对浮点数的测试](#5-对浮点数的测试)
  * [6. PartialOrd和Ord](#6-PartialOrd和Ord)
    * [6.1 与PartialEq和Eq的关系](#61-与PartialEq和Eq的关系)
    * [6.2 基本性质](#62-基本性质)
    * [6.3 trait定义](#63-trait定义)
    * [6.4 可派生](#64-可派生)
    * [6.5 手动实现PartialOrd和Ord](#65-手动实现PartialOrd和Ord)
    * [6.6 比较不同的类型](#66-比较不同的类型)
    * [6.7 Rust基本类型如何实现PartialOrd和Ord](#67-Rust基本类型如何实现PartialOrd和Ord)
    * [6.8 为其他类型实现四大compare-trait](#68-为其他类型实现四大compare-trait)
<!-- TOC -->

## 前言

本文将围绕对象：PartialEq和Eq，以及PartialOrd和Ord，即四个Rust中重点的**Compare Trait**进行讨论并解释其中的细节，内容涵盖理论以及代码实现。

在正式介绍PartialEq和Eq、以及PartialOrd和Ord之前，本文会首先介绍它们所遵循的数学理论，也就是**相等关系**。
文章主要分两大部分，第一部分是第2\~5节主要讨论PartialEq和Eq，第二大部分为第6节主要讨论PartialOrd和Ord，内容描述可能具有先后顺序，建议按章节顺序阅读。

## 1. 数学中的相等关系

在初中数学中，会介绍到什么是相等关系（也叫等价关系），相等关系是一种基本的二元关系，它描述了两个对象之间的相等性质。它必须满足如下三个性质：

- 自反性（反身性）：自己一定等于自己，即`a=a`；
- 对称性：若有`a=b`，则有`b=a`；
- 传递性：若有`a=b`和`b=c`，则有`a=c`；

也就是说，满足这三个性质才叫满足（完全）相等关系。这很容易理解，就不过多解释。

### 1.1 部分相等关系

对于简单的整数类型、字符串类型，我们可以说它们具有完全相等关系，因为它们可以全方位比较（包含两个维度，第一个是类型空间中的任意值，第二个是每个值的任意成员属性）， 但是对于某些类型就不行了，**这些类型总是不满足其中一个维度**
。下面一起来看看：
> 以字符串为例，全方位比较的是它的每个字节值以及整个字符串的长度。

#### 0. 浮点数类型

在浮点数类型中有个特殊的值是NaN（Not-a-number），这个值与任何值都不等（包括自己），它直接违背了自反性。这个时候，我们就需要为浮点数定义一种部分相等关系，这主要是为了比较非NaN浮点数。

> NaN定义于IEEE 754-2008标准的5.2节“特殊值”（Special Values）中，除了NaN，另外两个特殊值是正无穷大（+infinity）、负无穷大（-infinity），不过这两个值满足自反性。

除了浮点数类型，数学中还有其他类型也不具有**通常意义上**的全等关系，比如集合类型、函数类型。

#### 1. 集合类型

假设有集合A={1,2,3}、B={1,3,2}，那么此时A和B是相等还是不相等呢？这就需要在不同角度去看待，当我们只关注集合中是否包含相同的元素时， 可以说它们相等，当我们还要严格要求元素顺序一致时，它们就不相等。

在实际应用中，由我们定义（Impl）了一种集合中的特殊相等关系，称为"集合的相等"，这个特殊关系（实现逻辑）中，我们只要求两个集合的元素相同，不要求其他。

#### 2. 函数类型

首先从浮点数的NaN角度来看函数，假设有函数A=f(x)、B=f(y)，若x=y，那显然A的值也等于B，但是如果存在一个参数z是无意义的呢，意思是f(z)是无结果的或结果非法，那么此时可以说f(z)等于自身吗？
那显然是不行的。这个例子和浮点数的例子是一个意思。

然后从集合类型的角度再来看一次函数，假设有函数A=f(x)、B=g(x)，注意是两个不同的函数，当二者给定一个相同输入x产生相同结果时，此时f(x)和g(x)是相等还是不等呢？
与集合类似，实际应用中，这里也是由我们定义（Impl）了一种函数中的特殊相等关系，称为函数的相等。这个特殊关系（实现逻辑）中，我们只要求两个函数执行结果的值相同，不要求函数执行过程相同。

### 1.2 部分相等与全相等的关系

部分相等是全相等关系的子集，也就是说，如果两个元素具有相等关系，那它们之间也一定有部分相等关系。这在编程语言中的实现也是同样遵循的规则。

### 1.3 小结

数学中定义了（全）相等关系（等价关系）的三大性质，分别是自反性、对称性和传递性；但某些数据类型中的值或属性违背了三大性质，就不能叫做满足全相等关系， 此时只能为该类型实现**部分相等**关系。

**在部分相等关系中，用于比较的值也是满足三大性质的**，因为此时我们排除了那些特殊值。另外，部分相等是全相等关系的子集。

## 2. 编程与数学的关系

数学是一门研究数据、空间和变化的庞大学科，它提供了一种严谨的描述和处理问题的方式，而编程则是将问题的解决方法转化为计算机程序的过程，可以说，数学是问题的理论形式， 编程则是问题的代码形式，编程解决问题的依据来自数学。

所以说，编程语言的设计中也是大量运用了数学概念与模型的，本文关注的**相等关系**就是一个例子。

在Rust库中的`PartialEq`的注释文档中提到了[partial equivalence relations][0] 即部分相等关系这一概念，并且同样使用了浮点数的特殊值NaN来举例说明。

> `Eq`的注释文档则是提到了[equivalence relations][1]，并且明确说明了，对于满足`Eq`trait的类型，是一定满足相等关系的三大性质的。

## 3. PartialEq

### 3.1 trait定义

Rust中的PartialEq的命名明确地表达了它的含义，但如果我们忘记了数学中的相等关系，就肯定会对此感到疑惑。先来看看它的定义：

```rust
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}
```

在这个定义中，可以得到三个基本信息：

0. 这个trait包含2个方法，eq和ne，且ne具有默认实现，使用时开发者只需要实现eq方法即可（库文档也特别说明，若没有更好的理由，则不应该手动实现ne方法）；
1. PartialEq绑定的Rhs参数类型是`?Size`，即包括动态大小类型（DST）和固定大小类型（ST）类型（Rhs是主类型用来比较的类型）；
2. Rhs参数提供了默认类型即`Self`（和主类型一致），但也可以是**其他类型**，也就是说，实践中你甚至可以将`i32`与struct进行比较，只要实现了对应的`PartialEq`；

> Rust中的lhs和rhs指的是，"left-hand side"（左手边） 和 "right-hand side"（右手边）的参数。

### 3.2 对应操作符

这个比较简单，PartialEq和Eq一致，拥有的eq和ne方法分别对应`==`和`!=`两个操作符。Rust的大部分基本类型如整型、浮点数、字符串等都实现了PartialEq， 所以它们可以使用`==`和`!=`进行相等性比较。

### 3.3 可派生

英文描述为Derivable，即通过`derive`宏可以为自定义复合类型（struct/enum/union类型）自动实现PartialEq，用法如下：

```rust
#[derive(PartialEq)]
struct Book {
    name: String,
}

#[derive(PartialEq)]
enum BookFormat { Paperback, Hardback, Ebook }

#[derive(PartialEq)]
union T {
    a: u32,
    b: f32,
    c: f64,
}
```

需要注意的是，可派生的前提是这个复合类型下的所有成员字段都是支持PartialEq的，下面的代码说明了这种情况：

```rust
// #[derive(PartialEq)]  // 取消注释即可编译通过
enum BookFormat { Paperback, Hardback, Ebook }

// 无法编译！！！
#[derive(PartialEq)]
struct Book {
    name: String,
    format: BookFormat, // 未实现PartialEq
}
```

> 扩展：使用`cargo expand`命令可以打印出宏为类型实现的PartialEq代码。

### 3.4 手动实现PartialEq

以上一段代码为例，我们假设`BookFormat`是引用其他crate下的代码，无法为其添加derive语句（不能修改它），此时就需要手动为`Book`手动实现PartialEq，代码如下：

```rust
enum BookFormat { Paperback, Hardback, Ebook }

struct Book {
    name: String,
    format: BookFormat,
}

// 要求只要name相等则Book相等（假设format无法进行相等比较）
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn main() {
    let bk = Book { name: "x".to_string(), format: BookFormat::Ebook };
    let bk2 = Book { name: "x".to_string(), format: BookFormat::Paperback };
    assert!(bk == bk2); // 因为Book实现了PartialEq，所以可以比较相等性
}
```

### 3.5 比较不同的类型

根据上面的trait定义中，我们知道了只要在实现PartialEq时关联不同类型的Rhs参数，就能比较不同类型的相等性。示例代码如下：

```rust
#[derive(PartialEq)]
enum WheelBrand {
    Bmw,
    Benz,
    Michelin,
}

struct Car {
    brand: WheelBrand,
    price: i32,
}

impl PartialEq<WheelBrand> for Car {
    fn eq(&self, other: &WheelBrand) -> bool {
        self.brand == *other
    }
}

fn main() {
    let car = Car { brand: WheelBrand::Benz, price: 10000 };
    let wheel = WheelBrand::Benz;
    // 比较 struct和enum
    assert!(car == wheel);
    // assert!(wheel == car);  // 无法反过来比较
}
```

需要注意的是，代码片段中仅实现了Car与Wheel的相等性比较，若要反过来比较，还得提供反向的实现，如下：

```rust
impl PartialEq<Car> for WheelBrand {
    fn eq(&self, other: &Car) -> bool {
        *self == other.brand
    }
}
```

### 3.6 Rust基本类型如何实现PartialEq

上文说过，Rust的基本类型都实现了PartialEq，那具体是怎么实现的呢？是为每个类型都写一套impl代码吗？代码在哪呢？

如果你使用IDE，可以通过在任意位置按住ctrl键（视IDE而定）点击代码中的`PartialEq`以打开其在标准库中的代码文件`cmp.rs`，相对路径是`RUST_LIB_DIR/core/src/cmp.rs` 。
在该文件中可以找到如下宏代码：

```rust
mod impls {
    // ...
    macro_rules! partial_eq_impl {
        ($($t:ty)*) => ($(
            #[stable(feature = "rust1", since = "1.0.0")]
            #[rustc_const_unstable(feature = "const_cmp", issue = "92391")]
            impl const PartialEq for $t {
                #[inline]
                fn eq(&self, other: &$t) -> bool { (*self) == (*other) }
                #[inline]
                fn ne(&self, other: &$t) -> bool { (*self) != (*other) }
            }
        )*)
    }
    partial_eq_impl! {
        bool char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64
        }
    // ...
}
```

这里使用了Rust强大的宏特性（此处使用的是声明宏，还算简单），来为Rust的众多基本类型**快速**实现了PartialEq trait。如果你还不了解宏，可以暂且理解其是一种编写重复模式代码规则的编程特性，它可以减少大量重复代码。

## 4. Eq

理解了PartialEq，那Eq理解起来就非常简单了，本节的内容主体与PartialEq基本一致，所以相对简明。

### 4.1 trait定义

如下：

```rust
pub trait Eq: PartialEq<Self> {
    fn assert_receiver_is_total_eq(&self) {}
}
```

根据代码可以得到两个重要信息：

0. Eq是继承自PartialEq的；
1. Eq相对PartialEq只多了一个方法`assert_receiver_is_total_eq()`，并且有默认实现；

第一个，既然Eq继承自PartialEq，说明想要实现Eq，必先实现PartialEq。第二个是这个`assert_receiver_is_total_eq()`
方法了，简单来说，它是被derive语法内部使用的，用来断言类型的每个属性都实现了Eq特性，对于使用者的我们来说， 其实不用过多关注。

### 4.2 对应操作符

与PartialEq无差别，略。

### 4.3 可派生

与PartialEq的使用相似，只是要注意派生时，由于继承关系，Eq和PartialEq必须同时存在。

```rust
#[derive(PartialEq, Eq)] // 顺序无关
struct Book {
    name: String,
}
```

### 4.4 手动实现Eq

直接看代码：

```rust
enum BookFormat { Paperback, Hardback, Ebook }

struct Book {
    name: String,
    format: BookFormat,
}

// 要求只要name相等则Book相等（假设format无法进行相等比较）
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Book {}

fn main() {
    let bk = Book { name: "x".to_string(), format: BookFormat::Ebook };
    let bk2 = Book { name: "x".to_string(), format: BookFormat::Paperback };
    assert!(bk == bk2);
}
```

需要注意的是，必须先实现PartialEq，再实现Eq。另外，这里能看出的是，在比较相等性方面，Eq和PartialEq都是使用`==`和`!=`操作符，无差别感知。

### 4.5 比较不同的类型

与PartialEq无差别，略。

### 4.6 Rust基本类型如何实现Eq

与PartialEq一样，在相对路径为`RUST_LIB_DIR/core/src/cmp.rs`的文件中，存在如下宏代码：

```rust
mod impls {
    /*
        ... (先实现PartialEq)
        
    */

    // 再实现Eq
    macro_rules! eq_impl {
        ($($t:ty)*) => ($(
            #[stable(feature = "rust1", since = "1.0.0")]
            impl Eq for $t {}
        )*)
    }

    eq_impl! { () bool char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
}
```

## 5. 对浮点数的测试

目前在标准库中，笔者只发现有浮点数是只实现了PartialEq的（以及包含浮点数的复合类型），下面是浮点数的测试代码：

```rust
fn main() {
    fn check_eq_impl<I: Eq>(typ: I) {}
    // check_eq_impl(0.1f32); // 编译错误
    // check_eq_impl(0.1f64); // 编译错误

    let nan = f32::NAN;
    let infinity = f32::INFINITY;
    let neg_infinity = f32::NEG_INFINITY;
    assert_ne!(nan, nan); // 不等！
    assert_eq!(infinity, infinity); // 相等！
    assert_eq!(neg_infinity, neg_infinity);  // 相等！
}
```

## 6. PartialOrd和Ord

### 6.1 与PartialEq和Eq的关系

很多时候，当我们谈到PartialEq和Eq时，PartialOrd和Ord总是不能脱离的话题，因为它们都是一种二元比较关系，前两者是相等性比较，后两者是有序性（也可称大小性）比较。 前两者使用的操作符是`==`和`!=`
，后两者使用的操作符是`>`、`=`
、`<`，没错，PartialOrd和Ord的比较结果是**包含等于**的，然后我们可以基于这个有序关系来对数据进行排序（sort）。
> 重点：有序性包含相等性。

与PartialEq存在的原因一样，PartialOrd的存在的理由也是因为有一些类型是不具有**有序性**关系的（无法比较），比如浮点数、Bool、Option<T>、函数、闭包等类型。

**PartialEq和Eq、PartialOrd和Ord共同描述了Rust中任意类型的二元比较关系，包含相等性、有序性。** 所以在上文中，你可能也观察到PartialOrd和Ord的定义也位于`cmp.rs`文件中。

> 我们可以将PartialOrd和Ord直译为偏序和全序关系，因为这确实是它们要表达的含义。偏序和全序的概念来自离散数学，下文详解。

### 6.2 基本性质

PartialOrd和Ord也是满足一定的基本性质的，PartialOrd满足：

- 传递性：若有`a<b`、`b<c`，则`a<c`。且`>`和`==`也是一样的；
- 对立性：若有`a<b`，则`b>a`；

Ord基于PartialOrd，自然遵循传递性和对立性，另外对于任意两个元素，还满足如下性质：

- 确定性：必定存在`>`或`==`或`<`其中的一个关系；

### 6.3 trait定义

#### 1. PartialOrd trait

```rust
// 二元关系定义（<,==,>）
pub enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1,
}

pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
    fn lt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }
    fn le(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less | Equal))
    }
    fn gt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }
    fn ge(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }
}
```

基本信息：

0. PartialOrd继承自PartialEq，这很好理解，无法比较大小的类型也一定不能进行相等性比较；
1. 提供`partial_cmp()`方法用于主类型和可以是其他类型的参数比较，返回的`Option<Ordering>`，表示两者关系可以是无法比较的（None），那么这里我们就可以联想到Ord
   trait返回的肯定是`Ordering`（因为具有全序的类型不会存在无法比较的情况）；
2. 另外四个方法分别实现了对应的操作符:`<`, `<=`, `>`, `>=`，即实现了PartialOrd的类型可以使用这些操作符进行比较；除此之外，由于继承了PartialEq，所以还允许使用`==`,`!=`；

**请再次记住，不管是PartialOrd还是Ord，都包含相等关系。**

#### 2. Ord trait

```rust
pub trait Ord: Eq + PartialOrd<Self> {
    // 方法1
    fn cmp(&self, other: &Self) -> Ordering;

    // 方法2
    fn max(self, other: Self) -> Self
        where
            Self: Sized,
            Self: ~ const Destruct,
    {
        // HACK(fee1-dead): go back to using `self.max_by(other, Ord::cmp)`
        // when trait methods are allowed to be used when a const closure is
        // expected.
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => other,
            Ordering::Greater => self,
        }
    }

    // 方法3
    fn min(self, other: Self) -> Self
        where
            Self: Sized,
            Self: ~ const Destruct,
    {
        // HACK(fee1-dead): go back to using `self.min_by(other, Ord::cmp)`
        // when trait methods are allowed to be used when a const closure is
        // expected.
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => self,
            Ordering::Greater => other,
        }
    }

    // 方法4
    fn clamp(self, min: Self, max: Self) -> Self
        where
            Self: Sized,
            Self: ~ const Destruct,
            Self: ~ const PartialOrd,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}
```

基本信息：

0. `cmp`方法用于比较self与参数`other`的二元关系，返回`Ordering`类型（区别于PartialOrd.partial_cmp()返回的`Option<Ordering>`）；
1. Ord继承自Eq+PartialOrd，这也很好理解，具有全序关系的类型自然具有偏序关系；
2. 提供`min/max()`方法以返回self与参数`other`之间的较小值/较大值；
3. 额外提供`clamp()`方法返回输入的参数区间内的值；
4. 显然，由于继承了PartialOrd，所以实现了Ord的类型可以使用操作符`<`, `<=`, `>`, `>=`, `==`, `!=`；

> 对`Self: ~ const Destruct`的解释：位于where后即是类型约束，这里约束了`Self`类型必须是实现了`Destruct`trait的一个指向常量的裸指针。

> 全序和偏序的概念（来自离散数学）
>- 全序：即全序关系，自然也是一种二元关系。全序是指，集合中的任两个元素之间都可以比较的关系。比如实数中的任两个数都可以比较大小，那么“大小”就是实数集的一个全序关系。
>- 偏序：集合中只有部分元素之间可以比较的关系。比如复数集中并不是所有的数都可以比较大小，那么“大小”就是复数集的一个偏序关系。
>- 显然，全序关系必是偏序关系。反之不成立。

### 6.4 可派生

#### 1. PartialOrd derive

PartialOrd和Ord也是可以使用`derive`宏进行自动实现的，代码如下：

```rust
#[derive(PartialOrd, PartialEq)]
struct Book {
    name: String,
}

#[derive(PartialOrd, PartialEq)]
enum BookFormat { Paperback, Hardback, Ebook }
```

这里有几点需要注意：

0. 由于继承关系，所以必须同时派生PartialEq；
1. 与PartialEq相比，不支持为`union`类型派生；
2. 对struct进行派生时，大小顺序依据的是成员字段的字典序（字母表中的顺序，数字与字母比较则根据ASCII表编码，数字编码<字母编码；若比较多字节字符如中文，则转Unicode编码后再比较;
   实际上ASCII表中的字符编码与对应Unicode编码一致）；
3. 对enum进行派生时，大小顺序依据的是枚举类型的值大小，默认情况下，第一个枚举类型的值是1，向下递增1，所以第一个枚举最小；

下面使用代码对第2，3点举例说明：

```rust
#[derive(PartialOrd, PartialEq)]
struct Book {
    name: String,
}
assert!(Book { name: "a".to_string() } < Book { name: "b".to_string() });
assert!(Book { name: "b".to_string() } < Book { name: "c".to_string() });
// 字典序中，数字<字母（按ASCII编码排序）
assert!(Book { name: "1".to_string() } < Book { name: "2".to_string() });
assert!(Book { name: "2".to_string() } < Book { name: "a".to_string() });
// 字典序中，如果比较多字节字符，则先转为其Unicode的十六进制形式，然后逐字节比较
// 比如 中文 "曜" 和 "耀" 的Unicode编码分别为0x66DC和0x8000，所以前者小于后者
assert_eq!("曜", "\u{66dc}");
assert_eq!("耀", "\u{8000}");
assert!(Book { name: "曜".to_string() } < Book { name: "耀".to_string() });

#[derive(PartialOrd, PartialEq)]
enum BookFormat {
    Paperback,
    // 1
    Hardback,
    // 2
    Ebook,     // 3
}
assert!(BookFormat::Paperback < BookFormat::Hardback);
assert!(BookFormat::Hardback < BookFormat::Ebook);

#[derive(PartialOrd, PartialEq)]
enum BookFormat2 {
    // 手动指定枚举的值，则可以改变它们的大小顺序
    Paperback = 3,
    Hardback = 2,
    Ebook = 1,
}
assert!(BookFormat2::Paperback > BookFormat2::Hardback);
assert!(BookFormat2::Hardback > BookFormat2::Ebook);
```

对于字典序比较规则，还有一些特殊情况，如下：

- 如果元素A是元素B的前缀，则元素A<元素B；
- 空字符序列<非空字序列；

#### 2. Ord derive

```rust
 #[derive(Ord, Eq, PartialOrd, PartialEq)]
struct Book {
    name: String,
}

#[derive(Ord, Eq, PartialOrd, PartialEq)]
enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}
```

这里只需注意一点，那就是由于继承关系，Ord需要和Eq, PartialOrd, PartialEq同时派生。另外，根据前面所提到的，PartialOrd和Ord都支持`>=`, `<=`，这个要记得；

### 6.5 手动实现PartialOrd和Ord

#### 1. PartialOrd Impl

```rust
// 注意这里测试对象是Book3，不要为成员字段format即BookFormat3派生任何trait，模拟实际项目中无法修改成员字段特性的情况
enum BookFormat3 {
    Paperback,
    Hardback,
    Ebook,
}

struct Book3 {
    name: String,
    format: BookFormat3,
}

// -- 先得实现 PartialEq
impl PartialEq<Self> for Book3 {
    // tips：这里可以将<Self>省略
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
        // 这里假设format字段不要求比较
    }
}

// -- 才能实现 PartialOrd
impl PartialOrd for Book3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 直接调用name(String)的比较方法，如果成员字段也没有实现PartialOrd，那就得先为成员实现，这类情况很少
        self.name.partial_cmp(&other.name)
    }
}
```

#### 2. Ord Impl

```rust
// 测试对象：Book3
// - 这里同样没有使用任何derive，全手动实现，由于继承关系，需要实现四个trait
// - 注意：若存在任一成员字段(这里指   format字段)未实现PartialEq/Eq/PartialOrd，都是无法为Book3派生Ord的（派生时不会解析下面的手动impl）
enum BookFormat3 {
    Paperback,
    Hardback,
    Ebook,
}

struct Book3 {
    name: String,
    format: BookFormat3,
}

// -- 先实现 PartialEq
impl PartialEq for Book3 {
    fn eq(&self, other: &Book3) -> bool {
        self.name == other.name
        // 这里假设format字段不要求比较
    }
}

// -- 再实现 Eq
impl Eq for Book3 {}

// -- 再实现 Ord
impl Ord for Book3 {
    fn cmp(&self, other: &Book3) -> Ordering {
        // 直接调用name(String)的cmp方法（当需要实现Ord时，成员字段一般都实现了Ord，可直接调用其cmp方法）
        self.name.cmp(&other.name)
    }
}

// -- 最后实现 PartialOrd
impl PartialOrd for Book3 {
    fn partial_cmp(&self, other: &Book3) -> Option<Ordering> {
        // 直接调用上面实现的cmp方法
        Some(self.cmp(&other))
    }
}
```

阅读此代码需要注意几点：

0. 先读完代码注释；
1. 请注意是先实现Ord，再实现PartialOrd，理由是既然一开始就想要为类型实现Ord，说明类型是能够得出一个肯定结果的（非None），所以后实现PartialOrd直接调用Ord的`cmp()`；

### 6.6 比较不同的类型

这一节不贴代码了，留给读者去实现。具体实现手法可参考前面3.5节或4.5节中的内容。

### 6.7 Rust基本类型如何实现PartialOrd和Ord

#### 1. PartialOrd impl macro

我们以前面介绍过的同样的方式找到`cmp.rs`中PartialOrd的实现宏，代码如下：

```rust
mod impls {
    // ... 前面是PartialEq和Eq的宏实现

    macro_rules! partial_ord_impl {
        ($($t:ty)*) => ($(
            #[stable(feature = "rust1", since = "1.0.0")]
            #[rustc_const_unstable(feature = "const_cmp", issue = "92391")]
            impl const PartialOrd for $t {
                #[inline]
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                   // 注意看，此处是根据两个比较结果来得到最终结果，本质上是要求比较的值满足对立性（浮点数NaN不满足，所以返回None）
                    match (*self <= *other, *self >= *other) {
                        (false, false) => None,
                        (false, true) => Some(Greater),
                        (true, false) => Some(Less),
                        (true, true) => Some(Equal),
                    }
                }
                #[inline]
                fn lt(&self, other: &$t) -> bool { (*self) < (*other) }
                #[inline]
                fn le(&self, other: &$t) -> bool { (*self) <= (*other) }
                #[inline]
                fn ge(&self, other: &$t) -> bool { (*self) >= (*other) }
                #[inline]
                fn gt(&self, other: &$t) -> bool { (*self) > (*other) }
            }
        )*)
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_cmp", issue = "92391")]
    impl const PartialOrd for () {
        #[inline]
        fn partial_cmp(&self, _: &()) -> Option<Ordering> {
            Some(Equal)
        }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_cmp", issue = "92391")]
    impl const PartialOrd for bool {
        #[inline]
        fn partial_cmp(&self, other: &bool) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    partial_ord_impl! { f32 f64 }
}
```

这里要注意一下几点：

0. 代码中定义的宏`partial_ord_impl!`是通过两个比较结果来得到最终结果（看注释）；
1. 这个宏除了应用在了浮点数类型上，还应用在了`()`和`bool`类型。浮点数类型不必多说，单元类型是一种单值类型用于排序的情况也比较少，为bool类型实现这个trait的原因是：
   有时我们需要对包含bool类型成员的struct或enum进行排序，所以需要为其实现PartialOrd（注意其实现也是调用`self.cmp()`）；

> 这里的`impl const`中的const关键字意味着标记这个trait实现是编译时常量（编译时优化），以保证运行时不会有额外开销。这里是因为`fn partial_cmp()`的实现没有修改任何数据才可以加`const`，当然还有其他要求例如不能使用动态分配的内存（例如 Box 或 Vec）、不能调用非 const 函数等；

#### 2. Ord impl macro

```rust
mod impls {
    // ... 前面是PartialEq/Eq/PartialOrd的宏实现

    macro_rules! ord_impl {
        ($($t:ty)*) => ($(
            #[stable(feature = "rust1", since = "1.0.0")]
            #[rustc_const_unstable(feature = "const_cmp", issue = "92391")]
            impl const PartialOrd for $t {
                #[inline]
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
                #[inline]
                fn lt(&self, other: &$t) -> bool { (*self) < (*other) }
                #[inline]
                fn le(&self, other: &$t) -> bool { (*self) <= (*other) }
                #[inline]
                fn ge(&self, other: &$t) -> bool { (*self) >= (*other) }
                #[inline]
                fn gt(&self, other: &$t) -> bool { (*self) > (*other) }
            }

            #[stable(feature = "rust1", since = "1.0.0")]
            #[rustc_const_unstable(feature = "const_cmp", issue = "92391")]
            impl const Ord for $t {
                #[inline]
                fn cmp(&self, other: &$t) -> Ordering {
                    // The order here is important to generate more optimal assembly.
                    // See <https://github.com/rust-lang/rust/issues/63758> for more info.
                    if *self < *other { Less }
                    else if *self == *other { Equal }
                    else { Greater }
                }
            }
        )*)
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_cmp", issue = "92391")]
    impl const Ord for () {
        #[inline]
        fn cmp(&self, _other: &()) -> Ordering {
            Equal
        }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_cmp", issue = "92391")]
    impl const Ord for bool {
        #[inline]
        fn cmp(&self, other: &bool) -> Ordering {
            // Casting to i8's and converting the difference to an Ordering generates
            // more optimal assembly.
            // See <https://github.com/rust-lang/rust/issues/66780> for more info.
            match (*self as i8) - (*other as i8) {
                -1 => Less,
                0 => Equal,
                1 => Greater,
                // SAFETY: bool as i8 returns 0 or 1, so the difference can't be anything else
                _ => unsafe { unreachable_unchecked() },
            }
        }
    }

    ord_impl! { char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
}
```

这里需要了解一下几点：

0. 实现Ord的时候需要同时实现PartialOrd，不要求实现的顺序。PartialOrd的`partial_cmp()`内部调用的是Ord的`cmp()`，理由前面讲过；
1. 同样为`()`和bool类型实现了Ord；
2. 为大部分基本类型`char usize u8 u16 ...`实现了Ord；

### 6.8 为其他类型实现四大compare-trait

这里指的其他类型是`!`、`不可变借用类型`、`可变借用类型`，具体实现代码就在源码中刚刚看的宏`ord_impl!`下方，此处就不再赘述。


[0]: https://en.wikipedia.org/wiki/Partial_equivalence_relation

[1]: https://en.wikipedia.org/wiki/Equivalence_relation