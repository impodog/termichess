# TermiChess

A simple chess game in the terminal. Supporting local and remote multiplayer.

## How to execute

```bash
cargo run --release
```

## How to play

This game is played with two players. Each player performs a move once at a time. For chess references, see [Chess](https://en.wikipedia.org/wiki/Chess).

## Commands

To move a piece, you need to type a movement code. It specifies which piece to move and where to move it.

A legal position code is composed by two letters and two numbers. The first letter is the column (a-h), the second letter is the row (1-8).

Special pieces have their own code:
- King: `K`
- Queen: `Q`
- Rook: `R`
- Bishop: `B`
- Knight: `N`
- Pawn: Nothing or `P`

For example, the code `e2e4`, or `e4` for short moves the pawn at `e2` to `e4`. And the code `Qd1h5` `Qh5` moves the queen to `h5`.

To *capture* a piece you need to type an `x` between the piece code and the destination code. For example, `e4xd5` or `xd5` captures the piece at `d5` with the pawn at `e4`.

To *castle*, you need to type `0-0` for kingside castling and `0-0-0` for queenside castling.

To *promote* a pawn, you need to type the destination code followed with `=` and the piece code. For example, `e8=Q` moves the pawn to `e8` and promotes it to a queen.

Specially, to do en passant, you only need to type the destination code *without* the `x`. For example, `e5d6` or `d6` does en passant.

To *resign* the game, you need to type `resign`. Or, to *offer/accept a draw*, you need to type `draw`.

## Configurations

You can enhance your game experience by modifying the `termichess.toml` file, under where you start the executable.

Possible configurations are:
| Key | Description | Default |
| --- | --- | --- |
| `unicode` | Whether to use unicode symbols for chess pieces. If not, ascii characters are used. | `false` |
| `address` | The default address of the remote server to use. | `http://127.0.0.1:8080` |

## Playing Online

### Server

To play online, you need to have a server running. You can use the server provided in the `server` directory. To run it, you need to have Rust installed. Then, you can run the following command:

```bash
# server/
cargo run --release
```

### Client

After starting the server, you can play online by setting the `address` configuration to the server's address(default port is 8080). Then, enter one room code (e.g. `my-chess-room` `impodog's room` `Room1`), and wait for your friend to join the same room. Then, you can play with your friend online.
