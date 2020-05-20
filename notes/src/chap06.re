= 第6章 項の名無し表現

== 演習 6.1.1.

//texequation[611][]{
\begin{array}{lll}
c_0 &=& \lambda.\lambda.\ 0; \\
c_2 &=& \lambda.\lambda.\ 1\ (1\ 0); \\
\tt{plus} &=& \lambda.\lambda.\lambda.\lambda.\ 3\ 1\ (2\ 1\ 0); \\
\tt{fix} &=& \lambda.(\lambda.\ 1(\lambda.\ (1\ 1)\ 0))(\lambda.\ 2\ (\lambda.\ (0\ 0)1)); \\
\tt{foo} &=& (\lambda.(\lambda.\ 0))(\lambda.\ 0);
\end{array}
//}

自由変数を高々@<m>{n}含む項をn項と呼ぶ。

== 演習 6.2.2.

//texequation[621][]{
\begin{array}{l}
\lambda.\lambda.\ 1\ (0\ 4) \\
\lambda.\ 0\ 3\ (\lambda.\ 0\ 1\ 4)
\end{array}
//}

== 演習 6.2.5.

まずはインデックスの割り振り。

次は定義6.2.4.に従って代入をする。

代入は代入される変数はラムダ抽象の数で1上がるというインデックスの割り振りと同じ動作をする。これにより、インデックスの割り振りと代入される変数に割り振られたインデックスが対応する。

また、代入する値はシフトしていて複雑そうに見えるがこれも結局代入する値に出てくるインデックスを1増やしているだけ。これにより代入後もインデックスを割り振る前の変数に戻せる。

//texequation[625][]{
\begin{aligned}
(1)\ &[b \mapsto a](b\ (\lambda{x}.\lambda{y}.b)) \\
&= [0 \mapsto 1](0\ (\lambda.\lambda.2)) \\
&= 1\ (\lambda.\lambda.\ 3) \\
\
(2)\ &[b \mapsto a\ (\lambda{z}.a)](b\ (\lambda{x}.b)) \\
&= [0 \mapsto 1\ (\lambda.\ 2)](1\ (\lambda.\ 1)) \\
\end{aligned}
//}

== 評価

簡約によってラムダ抽象が1つ消える。最初のインデックスの割り振りとずれるので、代入後は-1シフトする。ただし、全体を-1するために、代入する消えるラムダ抽象に関係ない値も-1シフトするので、代入前に事前に代入する値を+1シフトする。

