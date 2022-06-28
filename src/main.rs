
macro_rules! unless {
        ($arg: expr, $branch: expr) => (if !$arg {$branch})
    }


fn main() {
    t (a ,b) = (1, 2);
    unless!(a < b, b - a);
}