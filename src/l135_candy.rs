/*
135. 分发糖果
老师想给孩子们分发糖果，有 N 个孩子站成了一条直线，老师会根据每个孩子的表现，预先给他们评分。

你需要按照以下要求，帮助老师给这些孩子分发糖果：

每个孩子至少分配到 1 个糖果。
相邻的孩子中，评分高的孩子必须获得更多的糖果。
那么这样下来，老师至少需要准备多少颗糖果呢？

示例 1:

输入: [1,0,2]
输出: 5
解释: 你可以分别给这三个孩子分发 2、1、2 颗糖果。
示例 2:

输入: [1,2,2]
输出: 4
解释: 你可以分别给这三个孩子分发 1、2、1 颗糖果。
     第三个孩子只得到 1 颗糖果，这已满足上述两个条件。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/candy
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
一遍遍历法:
1. 首先原则是贪心,尽可能的少给
2. 分数看成从低到高,再从高到低的变化过程
3. 要求最高点要同时满足左右两边的需求
4. 如果碰到相等连续相等的两个数,则当做一个从高到低的变化

只需计数单调增和单调减的个数即可,不用关心他们彼此的差距.
特殊情况:
就是连续相等的情况
比如1,2,3,3,2,1
那么第二个三和第一个三只能一样多
如果是1,2,3,3,1,那么第二个三应该给2个
如果是1,2,3,3,3,1,那么第一个3是三个,第二个3只需1个,第三个3则是2个
如果是1,2,2,3,那么第一个2则是2个,第二个2只需1个,从而总数只需4个
*/
struct Solution {}
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        0
    }
}
#[cfg(test)]
mod test {

    #[test]
    fn test() {}
}
