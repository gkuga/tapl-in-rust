= 型無しラムダ計算

簡約基は関数適用の形をしていて、右側を引数と呼んでいる。変数かラムダ抽象は値と呼んでいるよう。ラムダ計算における項はラムダ項と呼ぶ。

== 演習 5.2.1.

//texequation[or][]{
\begin{array}{lll}
\text{or} &=& \lambda{b}.\ \lambda{c}.\ b\ \text{tru}\ c \\
\text{not} &=& \lambda{b}.\ b\ \text{fls}\ \text{tru}
\end{array}
//}

== 演習 5.2.2.

@<m>{\text{scc\}}は外側に@<m>{s}をつけて、@<m>{\text{scc2\}}は@<m>{z}を@<m>{(s\ z)}にして1増やす。

//texequation[scc][]{
\text{scc2} = \lambda{n}.\ \lambda{s}.\ \lambda{z}.\ n\ s\ (s\ z)
//}

@<m>{\text{plus\}}は@<eq>{plus}の@<m>{m}の@<m>{z}に@<m>{n}を持ってきてくっつける感じ。

//texequation[plus][]{
\text{plus} = \lambda{m}.\ \lambda{n}.\ \lambda{s}.\ \lambda{z}.\ m\ s\ (n\ s\ z)
//}

@<m>{\text{timse\}}は@<m>{c_i}で@<m>{s}が@<m>{i}回出てくることを利用して、@<m>{\text{plus\}\ n}を@<m>{i}個作ることで掛け算にしてる。

//texequation[plus2][]{
\text{times} = \lambda{m}.\ \lambda{n}.\ m\ (\text{plus}\ n)\ c_0
//}

== 計算体系の拡張

@<m>{\lambda \equiv \lambda{\tt{\bf NB\}\}}のよう。

== 代入時のバグ

//texequation[beta][]{
(\lambda{x}.\ t_{12})t_2 \longrightarrow [x \mapsto t_2]t_{12}
//}

@<eq>{beta}は@<m>{t_{12\}}の中の自由変数@<m>{x}を全て@<m>{t_2}で置き換えることによって得られる項を表している。しかし@<m>{t_{12\}}の中の束縛変数名によって、束縛変数が自由変数になったり、自由変数が束縛変数になったりしてしまう。束縛変数名に関係なく得られる結果が同じでいてほしいので、慣習的にそのような時はアルファ変換によって束縛変数名を付け替える。その慣習を用いて、代入操作は定義5.3.5.のように定義される。

== 図 5-3

メタ変数の選択が評価順の制御をしているとは、@<m>{\text{E-A\small{PP\}\}1}で@<m>{t_1}を値に評価し@<m>{\text{E-A\small{PP\}\}2}で@<m>{t_2}を値に評価する。最後に@<m>{\text{E-A\small{PP\}\normalsize{A\}\small{BS\}\}}で関数の適用をするというもの。これは値評価の評価順になる。

