= 第7章

== untyped

ctxの使い方がややこしい。インデックスの割り振りをパース時にするが、そのためにctxを使っている。

mainでEvalかBindで分岐していて、Bind時はパース時と同様にctxに自由変数を入れている。そして、入れた変数を出力している。評価（Eval）時にctxを使ってはいない。

=== パース時

ctxはトップレベルで空で渡される。再帰的に順に渡されていき、EOFに達した時は全てのCommandを通過している。そしてトップレベルにはそのctxが返される。Commandではctxを渡すと基本的にTermを返すが、LCID Binderの時はLCIDをctxに入れて返しているので、全体でバインディングしたLCIDは通過後のctxに含まれる。バインディングが自由変数の宣言だとすれば、宣言後はctxに自由変数が含まれるので、自由変数が使える。つまりctxが宣言した自由変数かチェックするのに使える。

再帰を戻る時に変更後のctxが利用されているのは自由変数のバインディング時のみ。再帰を潜る時に変更後のctxを利用しているのはラムダ抽象にマッチし、そのBody部に再帰で潜るとき。つまり、ラムダ抽象の束縛変数がctxに含まれることになり、トップレベルで宣言された自由変数と束縛変数がctxに含まれる。ctxはスタックの用に使われているので、ラムダ抽象のBody部では変数を見つけると一番最近にctxに入った束縛変数をまずチェックして、見つけるとIndexとして0を割り振る。なければ+1していく。最終的に自由変数にもなければ未宣言ということになる。

=== 評価時
ctxはトップレベルで空で渡される。今度はfold_leftで順に渡される。そしてこの時点ではctxを使っていない。