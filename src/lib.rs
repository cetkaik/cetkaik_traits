use cetkaik_fundamental::{Color, Profession};

/// A trait that signifies that you can use it as a `Board` with an absolute coordinate
/// ／絶対座標付きの `Board` として扱える型を表すトレイト
pub trait IsAbsoluteBoard: IsBoard {
    /// The initial arrangement of the official (yhuap) rule
    fn yhuap_initial() -> Self;
}

/// A trait that signifies that you can use it as a `Board`
/// ／`Board` として扱える型を表すトレイト
pub trait IsBoard {
    /// A type that represents the piece
    type PieceWithSide: Copy;
    /// A type that represents the coordinate
    type Coord: Copy + std::fmt::Debug;

    /// peek
    fn peek(&self, c: Self::Coord) -> Option<Self::PieceWithSide>;
    /// pop
    fn pop(&mut self, c: Self::Coord) -> Option<Self::PieceWithSide>;
    /// put either a piece or a `None`
    fn put(&mut self, c: Self::Coord, p: Option<Self::PieceWithSide>);
    /// assert that the square is empty
    fn assert_empty(&self, c: Self::Coord);
    /// assert that the square is occupied
    fn assert_occupied(&self, c: Self::Coord);
    /// Moves the piece located at `from` to an empty square `to`.
    /// # Panics
    /// Should panics if either:
    /// - `from` is unoccupied
    /// - `to` is already occupied
    fn mov(&mut self, from: Self::Coord, to: Self::Coord) {
        self.pop(from).map_or_else(
            || panic!("Empty square encountered at {from:?}"),
            |src_piece| {
                self.assert_empty(to);
                self.put(to, Some(src_piece));
            },
        );
    }
}

/// A trait that signifies that you can use it as a `Field` in absolute coordinates
/// ／絶対座標で書かれた `Field` として扱える型を表すトレイト
pub trait IsAbsoluteField: IsField {
    /// The initial arrangement of the official (yhuap) rule
    fn yhuap_initial() -> Self;
}

/// A trait that signifies that you can use it as a `Field`
/// ／`Field` として扱える型を表すトレイト
pub trait IsField {
    /// A type that represents the board
    type Board: IsBoard<PieceWithSide = Self::PieceWithSide, Coord = Self::Coord>;
    /// A type that represents the coordinate
    type Coord: Eq + std::fmt::Debug;
    /// A type that represents the piece
    type PieceWithSide;
    /// A type that represents the side
    type Side;

    /// Moving a piece and taking it if necessary
    /// # Errors
    /// - `from` is unoccupied
    /// - `from` has Tam2
    /// - `to` has Tam2
    /// - `from` does not belong to `whose_turn`
    fn move_nontam_piece_from_src_to_dest_while_taking_opponent_piece_if_needed(
        &self,
        from: Self::Coord,
        to: Self::Coord,
        whose_turn: Self::Side,
    ) -> Result<Self, &'static str>
    where
        Self: std::marker::Sized;

    /// Remove a specified piece from one's hop1zuo1 and place it at `dest`;
    /// if none is found, or if `dest` is already occupied, return `None`.
    /// ／手駒から指定の駒を削除し、盤面に置く。指定の駒が手駒に見当たらないか、`dest` が既に埋まっているなら `None`。
    fn search_from_hop1zuo1_and_parachute_at(
        &self,
        color: Color,
        prof: Profession,
        side: Self::Side,
        dest: Self::Coord,
    ) -> Option<Self>
    where
        Self: std::marker::Sized;

    /// Immutably borrows the board
    fn as_board(&self) -> &Self::Board;

    /// Mutably borrows the board
    #[must_use]
    fn as_board_mut(&mut self) -> &mut Self::Board;
}

pub trait CetkaikRepresentation {
    type Perspective: Copy + Eq;

    type AbsoluteCoord: Copy + Eq + core::fmt::Debug;
    type RelativeCoord: Copy + Eq;

    type AbsoluteBoard: Clone
        + core::fmt::Debug
        + IsAbsoluteBoard<PieceWithSide = Self::AbsolutePiece, Coord = Self::AbsoluteCoord>;
    type RelativeBoard: Copy;

    type AbsolutePiece: Copy + Eq;
    type RelativePiece: Copy + Eq;

    type AbsoluteField: Clone
        + core::fmt::Debug
        + IsField<
            PieceWithSide = Self::AbsolutePiece,
            Coord = Self::AbsoluteCoord,
            Side = cetkaik_fundamental::AbsoluteSide,
        > + IsAbsoluteField;
    type RelativeField;

    // type AbsoluteSide: Copy + Eq + core::fmt::Debug + core::ops::Not;
    type RelativeSide: Copy + Eq;
    fn to_absolute_coord(coord: Self::RelativeCoord, p: Self::Perspective) -> Self::AbsoluteCoord;
    fn add_delta(
        coord: Self::RelativeCoord,
        row_delta: isize,
        col_delta: isize,
    ) -> Option<Self::RelativeCoord>;
    fn relative_get(
        board: Self::RelativeBoard,
        coord: Self::RelativeCoord,
    ) -> Option<Self::RelativePiece>;
    fn relative_clone_and_set(
        board: &Self::RelativeBoard,
        coord: Self::RelativeCoord,
        p: Option<Self::RelativePiece>,
    ) -> Self::RelativeBoard;
    fn absolute_get(
        board: &Self::AbsoluteBoard,
        coord: Self::AbsoluteCoord,
    ) -> Option<Self::AbsolutePiece>;
    fn is_tam_hue_by_default(coord: Self::RelativeCoord) -> bool;
    fn relative_tam2() -> Self::RelativePiece;
    fn absolute_tam2() -> Self::AbsolutePiece;
    fn is_upward(s: Self::RelativeSide) -> bool;
    fn match_on_piece_and_apply<U>(
        piece: Self::RelativePiece,
        f_tam: &dyn Fn() -> U,
        f_piece: &dyn Fn(Profession, Self::RelativeSide) -> U,
    ) -> U;
    fn empty_squares_relative(current_board: &Self::RelativeBoard) -> Vec<Self::RelativeCoord>;
    fn empty_squares_absolute(current_board: &Self::AbsoluteBoard) -> Vec<Self::AbsoluteCoord>;
    fn hop1zuo1_of(
        side: cetkaik_fundamental::AbsoluteSide,
        field: &Self::AbsoluteField,
    ) -> Vec<cetkaik_fundamental::ColorAndProf>;
    fn as_board_absolute(field: &Self::AbsoluteField) -> &Self::AbsoluteBoard;
    fn as_board_mut_absolute(field: &mut Self::AbsoluteField) -> &mut Self::AbsoluteBoard;
    fn as_board_relative(field: &Self::RelativeField) -> &Self::RelativeBoard;
    fn is_water_relative(c: Self::RelativeCoord) -> bool;
    fn is_water_absolute(c: Self::AbsoluteCoord) -> bool;
    fn loop_over_one_side_and_tam(
        board: &Self::RelativeBoard,
        side: Self::RelativeSide,
        f_tam_or_piece: &mut dyn FnMut(Self::RelativeCoord, Option<Profession>),
    );
    fn to_relative_field(field: Self::AbsoluteField, p: Self::Perspective) -> Self::RelativeField;
    fn to_relative_side(side: cetkaik_fundamental::AbsoluteSide, p: Self::Perspective) -> Self::RelativeSide;
    fn get_one_perspective() -> Self::Perspective;
    fn absolute_distance(a: Self::AbsoluteCoord, b: Self::AbsoluteCoord) -> i32;
    fn absolute_same_direction(
        origin: Self::AbsoluteCoord,
        a: Self::AbsoluteCoord,
        b: Self::AbsoluteCoord,
    ) -> bool;
    fn has_prof_absolute(piece: Self::AbsolutePiece, prof: Profession) -> bool;
}
