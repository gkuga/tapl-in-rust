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

@<m>{t \in S_i \to t \in S_{i+1\}}を示す。