use ow_workshop::prelude::*;

fn takes_num(_: impl IntoExpr<Num>) {}
fn takes_players(_: impl IntoExpr<Players>) {}
fn takes_vector(_: impl IntoExpr<Vector>) {}

fn main() {
    let int: Expr<Int> = 3.into();
    let _: Expr<Num> = int.clone().into();
    takes_num(int);
    takes_num(1.5);
    takes_num(2u32);

    let player: Expr<Player> = null();
    let players: Expr<Array<Player>> = null();
    let _: Expr<Players> = player.clone().into();
    let _: Expr<Players> = players.clone().into();
    takes_players(player);
    takes_players(players);

    let pos: Expr<Position> = null();
    takes_vector(pos);
    takes_vector(vect(1, 2, 3));

    let hero: Expr<Hero> = Hero::Ana.into();
    let _: Expr<Any> = hero.into();
}
