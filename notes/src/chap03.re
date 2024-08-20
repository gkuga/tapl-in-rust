= 型無し算術式

== 定義 3.2.1.の補足

最小の@<m>{\mathcal{T\}}とは(1)(2)(3)を満たす@<m>{\mathcal{T\}}の真部分集合@<m>{\subsetneq}が存在しないということである。ペアノの公理などでこの表現が使われる。

== 演習 3.2.4.

@<m>{S_i}の要素数は@<m>{|S_i| = |S_{i-1\}|^3 + 3|S_{i-1\}| + 3}と表せる。@<m>{S_3}は@<eq>{s3-size}となる。ここで@<m>{S_2}は@<eq>{s2-size}である。

//texequation[s3-size][@<m>{S_3}の要素数]{
\begin{aligned}
|S_3| &= |S_{2}|^3 + 3|S_{2}| + 3 \\
|S_3| &= 39^3 + 3\times3 + 3 \\
|S_3| &= 59439
\end{aligned}
//}

//texequation[s2-size][@<m>{S_2}の要素数]{
\begin{aligned}
|S_2| &= |S_{1}|^3 + 3|S_{1}| + 3 \\
|S_2| &= 3^3 + 3\times3 + 3 \\
|S_2| &= 39
\end{aligned}
//}

== 演習 3.2.5.

@<m>{j=0}のときは@<m>{S_0=\emptyset}なので@<eq>{t-def0}が成り立つ。

//texequation[t-def0][]{
S_0 \subseteq S_1
//}

@<eq>{t-assum0}と仮定する。

//texequation[t-assum0][]{
i=j+1 \land j \ge 0 \to S_j \subseteq S_{i}
//}

この時に@<m>{t \in S_i \to t \in S_{i+1\}}を示す。@<m>{j > 0}の時、@<m>{t \in S_i}は定義より以下の３通りのいずれかである。

//texequation[t-def1][]{
t \in \{true,\ false,\ 0\}
//}

//texequation[t-def2][]{
t \in \{succ\ t_1,\ pred\ t_1,\ iszero\ t_1\ |\ t_1 \in S_j\}
//}

//texequation[t-def3][]{
t \in \{if\ t_1\ then\ t_2\ else\ t_3\ |\ t_1,\ t_2,\ t_3 \in S_j\}
//}

@<eq>{t-def1}の場合は定義より、@<m>{t \in S_{i+1\}}である。

@<eq>{t-def2}の場合は、例えば、@<m>{t}は@<m>{t_1}を@<m>{S_{i+1\}}の定義の@<eq>{t-def2}に対応する部分に入れたものと一緒になる。つまり@<m>{t_1 \in S_i \to t \in S_{i+1\}}である。ここで仮定より@<m>{S_j \subseteq S_{i\}}なので、@<m>{t_1 \in S_i}であり、@<eq>{t-def2}の場合も@<m>{t \in S_{i+1\}}である。

@<eq>{t-def3}の場合も@<m>{t}は@<m>{t_1,\ t_2,\ t_3}を@<m>{S_{i+1\}}の定義の@<eq>{t-def2}に対応する部分に入れたものと一緒になる。先と同じことが言えるので同様に、@<eq>{t-def3}の場合も@<m>{t \in S_{i+1\}}である。

以上と@<eq>{t-def0}と@<eq>{t-assum0}により帰納的に@<m>{S_i \subseteq S_{i+1\}}である。

== 命題 3.2.6.

=== 完全帰納法について

公理2.4.1が高校で習う数学的帰納法で基本的には１つを仮定するが、公理2.4.2の完全帰納法では@<m>{i<n}である全ての@<m>{i}を仮定している。オイラーグラフの定理の証明で使われている。

定理3.3.4で完全帰納法と構造的帰納法の例と、証明の仕方が載っている。

== 3.3.4. 項に関する帰納法の原理の補足

//quote{
...項の上で直接議論でき、数を経由するような遠回りの必要がないためである。
//}

定理 3.5.12.で出てくる停止性の証明に見るように、整礎集合では半順序関係があるので大小が定義され、整礎であるため、自然数で０に対応する極小元が存在する。つまり深さやサイズといった自然数に対応させると完全帰納法が使えるが、構造的帰納法では、整礎であることをそのまま利用して、数を経由せずにそのまま帰納法を適用するという意味。

ここでは部分項という言い方をしているが、部分項というのはつまり半順序関係でいうより小さい構造であり、カッコ書きされている「より小さな項すべて」というような表現がしっくりくるのだが、どうだろう。

== 定義 3.5.3.

木はだいたい逆向きに描かれるが、ここで出てくる導出木は葉が上の通常の方向を向いた木。提示された評価関係式が導出可能であることが証明された。つまり１ステップ評価関係であるということ。

== 演習 3.5.5.

== 定理 3.5.7.

//texequation[val][]{
すべての値は正規形である。
//}

どんなプログラミング言語も確かに、値が更に評価されることはない。これが成り立たない言語定義はこわれている。この章で定義する言語はこの逆も成り立つが、一般的には成り立たない。値ではないがそれ以上評価ができない正規形は、実行時エラーということである。これが未定義状態ということだろうか？

== 定理 3.5.8.

この言語体系では@<eq>{theorem358}が証明できる。

//texequation[theorem358][]{
tが正規形であるならば、tは値である。
//}

@<m>{V, \text{NF\}}をそれぞれこの言語体系の値の集合、正規形の集合としたとき、@<m>{t \notin V \to t \notin \text{NF\}}を証明することで、証明している。

この定理はこの言語体系では実行時エラーがおこらないことが証明されたということだろうか？この言語体系でエラーが起こったならば、コンパイラのバグか、コンピュータの故障かというようなことになるだろう。

== 演習 3.5.10.

== 定理 3.5.11.

定義 2.2.10.で整礎集合（と定義されているが、整列集合とも言う？別ものだろうか。）で、機械状態と集合の1対1対応（全単射）を定義し、状態遷移により対応する整列集合の要素が小さくなることを示すことで停止性を証明する。

== 演習 3.5.13.

== 演習 3.5.14.

== 定義 3.5.15.

ここで、値ではない正規形が出てくる。例えば、@<m>{succ true}は値ではない正規形で、この簡約（評価関係）は定義されていない。これを行き詰まり状態という。行き詰まり状態は実行時エラーを表す。新たな言語体系ではエラーが起こるということである。ただし、これはコンパイル時に検出できるということだろう。

== 演習 3.5.16.

数でない正規形をwrongという項に遷移するようにして、行き詰まり状態を間違い状態のようなものへ遷移するようにしている。

== 演習 3.5.17.

//texequation[bigstep][]{
t \longrightarrow^* v \equiv t \Downarrow v
//}

@<eq>{bigstep}を示すために@<eq>{proposition1}と@<eq>{proposition2}を順に証明する。

まず@<eq>{proposition1}について。

//texequation[proposition1][]{
P(t) = t \longrightarrow^* v \ \to \ t \Downarrow v
//}

@<m>{\text{depth\}(t)=0}の時、@<m>{t={true,false,0\}}であり@<m>{\text{B-V\small{ALUE\}\}}より@<eq>{proposition1}が成り立つ。よって@<eq>{assum0}である。

//texequation[assum0][]{
depth(t) = 0 \to P(t)
//}

@<eq>{assum1}と仮定する。

//texequation[assum1][]{
\text{depth}(r)<\text{depth}(s) \to P(r)
//}

@<eq>{assum2}の時：

//texequation[assum2][]{
s=\text{if}\ s_1\ \text{then}\ s_2\ \text{else}\ s_3 \land s_1 \longrightarrow^* true
//}

@<m>{s \longrightarrow^* s_2}である。

帰納法の仮定により、@<eq>{theorem1}と@<eq>{theorem2}が成り立つ。

//texequation[theorem1][]{
P(s_1) = s_1 \longrightarrow^* true \ \to \ s_1 \Downarrow true
//}

//texequation[theorem2][]{
P(s_2) = s_2 \longrightarrow^* v_2 \ \to \ s_2 \Downarrow v_2
//}

@<eq>{theorem1}と@<eq>{theorem1}と@<m>{\text{B-I\small{F\}\normalsize{T\}\small{RUE\}\}}により@<eq>{theorem3}が成り立つ。

//texequation[theorem3][]{
s \Downarrow v_2
//}

したがって@<m>{P(s)}が成り立つ。

@<eq>{assum3}の時：

//texequation[assum3][]{
s=\text{if}\ s_1\ \text{then}\ s_2\ \text{else}\ s_3 \land s_1 \longrightarrow^* false
//}

同様に@<m>{P(s)}が成り立つ。

@<m>{\text{succ, pred, iszero\}}においても同様の議論で@<m>{P(s)}が証明でき、結果@<eq>{proposition1}が成り立つ。

次に@<eq>{proposition2}について。

//texequation[proposition2][]{
Q(t) = t \Downarrow v \ \to \ t \longrightarrow^* v
//}

@<m>{T(s)}を@<m>{s}の任意の直接の部分項の集合とする。このとき@<eq>{assum4}と仮定する。

//texequation[assum4][]{
r \in T(s) \to Q(r)
//}

@<m>{s}が部分項を持たない時、つまり@<m>|s \in \{true,false,0\} \to Q(s)|は自明である。部分項からなる項の時を場合分けして@<m>{Q(s)}を証明する。

@<eq>{assum5}の時：

//texequation[assum5][]{
s=\text{if}\ s_1\ \text{then}\ s_2\ \text{else}\ s_3 \land s_1 \Downarrow true
//}

帰納法の仮定により、@<eq>{theorem4}と@<eq>{theorem5}が成り立つ。

//texequation[theorem4][]{
Q(s_1) = s_1 \Downarrow true \ \to \ s_1 \longrightarrow^* true
//}

//texequation[theorem5][]{
Q(s_2) = s_2 \Downarrow v_2 \ \to \ s_2 \longrightarrow^* v_2
//}

@<eq>{theorem4}と@<eq>{theorem5}により@<m>{Q(s)}が成り立つ。

...後で清書

== 演習 3.5.18.
