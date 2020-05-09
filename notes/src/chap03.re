= 第３章 型無し算術式

== 演習

=== 3.2.4

@<m>{S_i}の要素数は@<m>{|S_i| = |S_{i-1\}|^3 + 3|S_{i-1\}| + 3}と表せる。@<m>{S_3}は@<eq>{s3-size}となる。ここで@<m>{S_2}は@<eq>{s2-size}である。

//texequation[s3-size][@<m>{S_3}の要素数]{
\begin{array}{l}
|S_3| = |S_{2}|^3 + 3|S_{2}| + 3 \\
|S_3| = 39^3 + 3\times3 + 3 \\
|S_3| = 59439
\end{array}
//}

//texequation[s2-size][@<m>{S_2}の要素数]{
\begin{array}{lll}
|S_2| &=& |S_{1}|^3 + 3|S_{1}| + 3 \\
|S_2| &=& 3^3 + 3\times3 + 3 \\
|S_2| &=& 39
\end{array}
//}

=== 3.2.5

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

== 命題

=== 3.2.5

==== 完全帰納法について

公理2.4.1が高校で習う数学的帰納法で基本的には１つを仮定するが、公理2.4.2の完全帰納法では@<m>{i<n}である全ての@<m>{i}を仮定している。オイラーグラフの定理の証明で使われている。

定理3.3.4で完全帰納法と構造的帰納法の例と、証明の仕方が載っている。

==== 定義 3.2.1の補足

最小の@<m>{\mathcal{T\}}とは(1)(2)(3)を満たす@<m>{\mathcal{T\}}の真部分集合が存在しないということである。ペアノの公理などでこの表現が使われる。k