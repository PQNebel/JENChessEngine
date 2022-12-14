use super::*;

#[derive(PartialEq)]
pub enum MoveTypes {
    All,
    Quiescence
}

#[inline(always)]
pub fn generate_moves(game: &mut Game, move_types: MoveTypes) -> MoveList {
    let mut moves = MoveList::new();
    let mut from_sq: u8;
    let mut to_sq:   u8;

    let mut attacks: Bitboard;
    let mut quiet: Bitboard;

    let mut pawn_bitboard;
    let mut rook_bitboard;
    let mut knight_bitboard;
    let mut bishop_bitboard;
    let mut queen_bitboard;
    let mut king_bitboard;

    let opponent_occupancies: Bitboard;

    let rook;
    let knight;
    let bishop;
    let queen;
    let king;

    //Color specific
    //WHITE
    if game.active_player == Color::White {
        opponent_occupancies = game.black_occupancies;
        pawn_bitboard =     game.get_piece_bitboard(Piece::WhitePawn);
        rook_bitboard =     game.get_piece_bitboard(Piece::WhiteRook);
        knight_bitboard =   game.get_piece_bitboard(Piece::WhiteKnight);
        bishop_bitboard =   game.get_piece_bitboard(Piece::WhiteBishop);
        queen_bitboard =    game.get_piece_bitboard(Piece::WhiteQueen);
        king_bitboard =     game.get_piece_bitboard(Piece::WhiteKing);

        rook =   Piece::WhiteRook as u8;
        knight = Piece::WhiteKnight as u8;
        bishop = Piece::WhiteBishop as u8;
        queen =  Piece::WhiteQueen as u8;
        king =   Piece::WhiteKing as u8;

        //Pawn moves
        while !pawn_bitboard.is_empty() {
            from_sq = pawn_bitboard.extract_bit();
            to_sq = (from_sq as i8 - 8) as u8;
            //Quiet
            if move_types == MoveTypes::All && !game.all_occupancies.get_bit(to_sq) {
                //to_sq is empty
                if to_sq >= 8 {
                    //Quiet move
                    moves.add_move(Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::None as u8, false, false, false, false));

                    //Double push
                    to_sq = (to_sq as i8 - 8) as u8;
                    if !game.all_occupancies.get_bit(to_sq) && from_sq / 8 == 6 {
                        moves.add_move(Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::None as u8, false, true, false, false));
                    }
                }
                //Promotions
                else {
                    moves.add_move(Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteQueen as u8,  false, false, false, false));
                    moves.add_move(Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteKnight as u8, false, false, false, false));
                    moves.add_move(Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteRook as u8,   false, false, false, false));
                    moves.add_move(Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteBishop as u8, false, false, false, false))
                }
            }

            //Captures
            attacks = get_pawn_attack_table(from_sq, Color::White);

            //enpassant
            if game.enpassant_square != Square::None && !attacks.and(Bitboard::from_u64(1 << game.enpassant_square as u8)).is_empty(){
                moves.add_move(Move::new(from_sq, game.enpassant_square as u8, Piece::WhitePawn as u8, Piece::None as u8, true, false, true, false));
            }

            //Overlap with opponent occupancies
            attacks = attacks.and(game.black_occupancies);

            while !attacks.is_empty() {
                to_sq = attacks.extract_bit();
                //Regular captures
                if to_sq >= 8 {
                    moves.add_move(Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::None as u8, true, false, false, false));

                //Promotions
                } else {
                    moves.add_move(Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteQueen as u8,  true, false, false, false));
                    moves.add_move(Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteKnight as u8, true, false, false, false));
                    moves.add_move(Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteRook as u8,   true, false, false, false));
                    moves.add_move(Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteBishop as u8, true, false, false, false))
                }
            }
        }

        //Castling kingside
        if  move_types == MoveTypes::All && game.castling_ability & (CastlingAbility::WhiteKingSide as u8) != 0 &&              //castling ability
            (game.all_occupancies.and(Bitboard::from_u64(6917529027641081856))).is_empty() &&   //f1 and g1 are free. 6917529027641081856 is f1 and g1 set
            !game.is_square_attacked(Square::e1 as u8, Color::Black) &&                         //e1 is notunder attack
            !game.is_square_attacked(Square::f1 as u8, Color::Black) {                          //f1 is not under attack

                moves.add_move(Move::new(Square::e1 as u8, Square::g1 as u8, Piece::WhiteKing as u8, Piece::None as u8, false, false, false, true))
        }
        //Castling queen
        if  move_types == MoveTypes::All && game.castling_ability & (CastlingAbility::WhiteQueenSide as u8) != 0 &&             //castling ability
            (game.all_occupancies.and(Bitboard::from_u64(1008806316530991104))).is_empty() &&   //d1, c1 and b1 are free. 1008806316530991104 is f1 and g1 set
            !game.is_square_attacked(Square::e1 as u8, Color::Black) &&                         //e1 is notunder attack
            !game.is_square_attacked(Square::d1 as u8, Color::Black) {                          //d1 is not under attack

                moves.add_move(Move::new(Square::e1 as u8, Square::c1 as u8, Piece::WhiteKing as u8, Piece::None as u8, false, false, false, true))
        }
    }
    //BLACK
    else {
        opponent_occupancies = game.white_occupancies;
        pawn_bitboard = game.get_piece_bitboard(Piece::BlackPawn);
        rook_bitboard = game.get_piece_bitboard(Piece::BlackRook);
        knight_bitboard = game.get_piece_bitboard(Piece::BlackKnight);
        bishop_bitboard = game.get_piece_bitboard(Piece::BlackBishop);
        queen_bitboard = game.get_piece_bitboard(Piece::BlackQueen);
        king_bitboard = game.get_piece_bitboard(Piece::BlackKing);

        rook =   Piece::BlackRook as u8;
        knight = Piece::BlackKnight as u8;
        bishop = Piece::BlackBishop as u8;
        queen =  Piece::BlackQueen as u8;
        king =   Piece::BlackKing as u8;

        //Pawn moves
        while !pawn_bitboard.is_empty() {
            from_sq = pawn_bitboard.extract_bit();
            to_sq = (from_sq as i8 + 8) as u8;
            //Quiet
            if move_types == MoveTypes::All && !game.all_occupancies.get_bit(to_sq) {
                //to_sq is empty
                if to_sq <= 55 {
                    //Quiet move
                    moves.add_move(Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::None as u8, false, false, false, false));

                    //Double push
                    to_sq = (to_sq as i8 + 8) as u8;
                    if !game.all_occupancies.get_bit(to_sq) && from_sq / 8 == 1 {
                        moves.add_move(Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::None as u8, false, true, false, false));
                    }
                }
                //Promotions
                else {
                    moves.add_move(Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackQueen as u8,  false, false, false, false));
                    moves.add_move(Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackKnight as u8, false, false, false, false));
                    moves.add_move(Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackRook as u8,   false, false, false, false));
                    moves.add_move(Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackBishop as u8, false, false, false, false))
                }
            }

            //Captures
            attacks = get_pawn_attack_table(from_sq, Color::Black);

            //enpassant
            if game.enpassant_square != Square::None && !attacks.and(Bitboard::from_u64(1 << game.enpassant_square as u8)).is_empty(){
                moves.add_move(Move::new(from_sq, game.enpassant_square  as u8, Piece::BlackPawn as u8, Piece::None as u8, true, false, true, false));
            }

            //Overlap with opponent occupancies
            attacks = attacks.and(game.white_occupancies);

            while !attacks.is_empty() {
                to_sq = attacks.extract_bit();
                //Regular captures
                if to_sq <= 55 {
                    moves.add_move(Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::None as u8, true, false, false, false));

                //Promotions
                } else {
                    moves.add_move(Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackQueen as u8,  true, false, false, false));
                    moves.add_move(Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackKnight as u8, true, false, false, false));
                    moves.add_move(Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackRook as u8,   true, false, false, false));
                    moves.add_move(Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackBishop as u8, true, false, false, false))
                }
            }
        }

        //Castling kingside
        if  move_types == MoveTypes::All && game.castling_ability & (CastlingAbility::BlackKingSide as u8) != 0 &&              //castling ability
            (game.all_occupancies.and(Bitboard::from_u64(96))).is_empty() &&                    //f8 and g8 are free. 96 is f1 and g1 set
            !game.is_square_attacked(Square::e8 as u8, Color::White) &&                         //e8 is notunder attack
            !game.is_square_attacked(Square::f8 as u8, Color::White) {                          //f8 is not under attack

                moves.add_move(Move::new(Square::e8 as u8, Square::g8 as u8, Piece::BlackKing as u8, Piece::None as u8, false, false, false, true))
        }
        //Castling queen
        if  move_types == MoveTypes::All && game.castling_ability & (CastlingAbility::BlackQueenSide as u8) != 0 &&             //castling ability
            (game.all_occupancies.and(Bitboard::from_u64(14))).is_empty() &&                    //d8, c8 and b8 are free. 14 is f1 and g1 set
            !game.is_square_attacked(Square::e8 as u8, Color::White) &&                         //e8 is notunder attack
            !game.is_square_attacked(Square::d8 as u8, Color::White) {                          //d8 is not under attack

                moves.add_move(Move::new(Square::e8 as u8, Square::c8 as u8, Piece::BlackKing as u8, Piece::None as u8, false, false, false, true))
        }
    }

    //Knight attacks
    while !knight_bitboard.is_empty() {
        from_sq = knight_bitboard.extract_bit();

        //Raw attack table
        attacks = get_knight_attack_table(from_sq);

        if move_types == MoveTypes::All {
            //Extract only quiet moves and loop over them
            quiet = attacks.and(not(game.all_occupancies));
            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                moves.add_move(Move::new(from_sq, to_sq, knight, Piece::None as u8, false, false, false, false))
            }
        }

        //Extract only captures and loop over them
        attacks = attacks.and(opponent_occupancies);
        while !attacks.is_empty() {
            to_sq = attacks.extract_bit();
            moves.add_move(Move::new(from_sq, to_sq, knight, Piece::None as u8, true, false, false, false))
        }
    }

    //Bishop attacks
    while !bishop_bitboard.is_empty() {
        from_sq = bishop_bitboard.extract_bit();

        //Raw attack table
        attacks = get_bishop_attack_table(from_sq, game.all_occupancies);

        if move_types == MoveTypes::All {
            //Extract only quiet moves and loop over them
            quiet = attacks.and(not(game.all_occupancies));
            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                moves.add_move(Move::new(from_sq, to_sq, bishop, Piece::None as u8, false, false, false, false))
            }
        }       

        //Extract only captures and loop over them
        attacks = attacks.and(opponent_occupancies);
        while !attacks.is_empty() {
            to_sq = attacks.extract_bit();
            moves.add_move(Move::new(from_sq, to_sq, bishop, Piece::None as u8, true, false, false, false))
        }
    }

    //Rook attacks
    while !rook_bitboard.is_empty() {
        from_sq = rook_bitboard.extract_bit();

        //Raw attack table
        attacks = get_rook_attack_table(from_sq, game.all_occupancies);

        if move_types == MoveTypes::All {
            //Extract only quiet moves and loop over them
            quiet = attacks.and(not(game.all_occupancies));
            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                moves.add_move(Move::new(from_sq, to_sq, rook, Piece::None as u8, false, false, false, false))
            }
        }
        //Extract only captures and loop over them
        attacks = attacks.and(opponent_occupancies);
        while !attacks.is_empty() {
            to_sq = attacks.extract_bit();
            moves.add_move(Move::new(from_sq, to_sq, rook, Piece::None as u8, true, false, false, false))
        }
    }

    //Queen attacks
    while !queen_bitboard.is_empty() {
        from_sq = queen_bitboard.extract_bit();

        //Raw attack table
        attacks = get_queen_attack_table(from_sq, game.all_occupancies);

        if move_types == MoveTypes::All {
            //Extract only quiet moves and loop over them
            quiet = attacks.and(not(game.all_occupancies));
            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                moves.add_move(Move::new(from_sq, to_sq, queen, Piece::None as u8, false, false, false, false))
            }
        }
        //Extract only captures and loop over them
        attacks = attacks.and(opponent_occupancies);
        while !attacks.is_empty() {
            to_sq = attacks.extract_bit();
            moves.add_move(Move::new(from_sq, to_sq, queen, Piece::None as u8, true, false, false, false))
        }
    }

    //King attacks
    while !king_bitboard.is_empty() {
        from_sq = king_bitboard.extract_bit();

        //Raw attack table
        attacks = get_king_attack_table(from_sq,);

        if move_types == MoveTypes::All {
            //Extract only quiet moves and loop over them
            quiet = attacks.and(not(game.all_occupancies));
            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                moves.add_move(Move::new(from_sq, to_sq, king, Piece::None as u8, false, false, false, false))
            }
        }

        //Extract only captures and loop over them
        attacks = attacks.and(opponent_occupancies);
        while !attacks.is_empty() {
            to_sq = attacks.extract_bit();
            moves.add_move(Move::new(from_sq, to_sq, king, Piece::None as u8, true, false, false, false))
        }
    }

    moves
}

#[inline(always)]
pub fn is_legal(game: &Game, cmove: &Move) -> bool {
    let from_sq = cmove.from_square();
    let to_sq = cmove.to_square();
    let capture = cmove.is_capture();
    let piece_ind = cmove.piece() as usize;

    let mut copy = *game;

    //Peek make
    //Update all_occupancies
    copy.all_occupancies.unset_bit(from_sq);
    copy.bitboards[piece_ind].unset_bit(from_sq);
    copy.bitboards[piece_ind].set_bit(to_sq);
    copy.all_occupancies.set_bit(to_sq);
    
    if cmove.is_enpassant() {
        if copy.active_player == Color::White {
            copy.bitboards[Piece::BlackPawn as usize].unset_bit(to_sq + 8);
            copy.all_occupancies.unset_bit(to_sq + 8);
        }
        else {
            copy.bitboards[Piece::WhitePawn as usize].unset_bit(to_sq - 8);
            copy.all_occupancies.unset_bit(to_sq - 8);
        }
    }

    //Unset captured
    else if capture {
        let start;
        let end;
        if copy.active_player == Color::White {
            start = Piece::BlackPawn as usize;
            end = Piece::BlackKing as usize;
        }
        else {
            start = Piece::WhitePawn as usize;
            end = Piece::WhiteKing as usize;
        }

        for bb in start..end {
            if copy.bitboards[bb].get_bit(to_sq) {
                copy.bitboards[bb].unset_bit(to_sq);
                break;
            }
        }
    }

    //Check check
    !copy.is_in_check(game.active_player)
}

#[cfg(test)]
mod move_gen_tests {
    use super::*;

    #[test]
    pub fn perft_test () {
        let mut game = Game::new_from_start_pos();
        let mut moves = generate_moves(&mut game, MoveTypes::All).legal_values(&mut game);
        make_move(&mut game, moves.iter().find(|m| m.from_square() == Square::a2 as u8 && m.to_square() == Square::a3 as u8 ).unwrap());
        moves = generate_moves(&mut game, MoveTypes::All).legal_values(&mut game);
        make_move(&mut game, moves.iter().find(|m| m.from_square() == Square::d7 as u8 && m.to_square() == Square::d6 as u8 ).unwrap());
        moves = generate_moves(&mut game, MoveTypes::All).legal_values(&mut game);
        make_move(&mut game, moves.iter().find(|m| m.from_square() == Square::b2 as u8 && m.to_square() == Square::b3 as u8 ).unwrap());
        moves = generate_moves(&mut game, MoveTypes::All).legal_values(&mut game);
        make_move(&mut game, moves.iter().find(|m| m.from_square() == Square::c8 as u8 && m.to_square() == Square::h3 as u8 ).unwrap());
        generate_moves(&mut game, MoveTypes::All).print();
        let pe = perft(&mut game, 1, true);
        game.all_occupancies.print();
        println!("Found total: {}", pe)

    }

    //Legal moves
    #[test]
    pub fn white_pawn_can_move_one_tile_forward() {
        let mut game = Game::new_from_start_pos();
        let moves = generate_moves(&mut game, MoveTypes::All);
        assert!(moves.contains(&Move::new_friendly(Square::a2, Square::a3, Piece::WhitePawn, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn black_pawn_can_move_one_tile_forward() {
        let mut game =  Game::new_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All);
        assert!(moves.contains(&Move::new_friendly(Square::c7, Square::c6, Piece::BlackPawn, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn white_pawn_can_correctly_double_push() {
        let mut game = Game::new_from_start_pos();
        let moves = generate_moves(&mut game, MoveTypes::All);
        assert!(moves.contains(&Move::new_friendly(Square::a2, Square::a4, Piece::WhitePawn, Piece::None, false, true, false, false)));
    }

    #[test]
    pub fn black_pawn_can_correctly_double_push() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All);
        assert!(moves.contains(&Move::new_friendly(Square::c7, Square::c5, Piece::BlackPawn, Piece::None, false, true, false, false)));
    }

    #[test]
    pub fn pawn_can_capture_on_both_diagonals() {
        let mut game = Game::new_from_fen("1k6/8/8/4p1b1/5P2/8/8/1K6 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All);
        assert!(moves.contains(&Move::new_friendly(Square::f4, Square::e5, Piece::WhitePawn, Piece::None, true, false, false, false)));
        assert!(moves.contains(&Move::new_friendly(Square::f4, Square::g5, Piece::WhitePawn, Piece::None, true, false, false, false)));
    }

    #[test]
    pub fn white_can_enpassant_capture_correctly() {
        let mut game = Game::new_from_fen("k7/8/8/4Pp2/8/8/8/K7 w - f6 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All);
        moves.print();
        assert!(moves.contains(&Move::new_friendly(Square::e5, Square::f6, Piece::WhitePawn, Piece::None, true, false, true, false)));
    }

    #[test]
    pub fn black_can_enpassant_capture_correctly() {
        let mut game = Game::new_from_fen("k7/8/8/8/8/pP6/8/7K b - b2 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All);
        assert!(moves.contains(&Move::new_friendly(Square::a3, Square::b2, Piece::BlackPawn, Piece::None, true, false, true, false)));
    }

    #[test]
    pub fn can_not_move_pawn_when_piece_in_the_way() {
        let mut game = Game::new_from_fen("k7/8/8/8/8/1N6/1P6/K7 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::b2);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    pub fn white_pawn_should_have_4_promotion_options_when_reaching_back_row() {
        let mut game = Game::new_from_fen("k7/2P5/8/8/8/8/8/K7 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::c7);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    pub fn black_pawn_should_have_4_promotion_options_when_reaching_back_row() {
        let mut game = Game::new_from_fen("k7/8/8/8/8/8/2p5/K7 b - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::c2);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    pub fn should_be_able_to_promote_on_back_row_capture() {
        let mut game = Game::new_from_fen("k2r4/2P5/8/8/8/8/8/K7 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::c7);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    pub fn white_knight_has_2_right_legal_moves_at_start() {
        let mut game = Game::new_from_start_pos();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::b1);
        assert!(moves.len() == 2);
        assert!(moves.contains(&Move::new_friendly(Square::b1, Square::a3, Piece::WhiteKnight, Piece::None, false, false, false, false)));
        assert!(moves.contains(&Move::new_friendly(Square::b1, Square::c3, Piece::WhiteKnight, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn king_can_move_in_all_directions() {
        let mut game = Game::new_from_fen("8/1K6/8/4k3/8/8/8/8 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::b7);
        assert!(moves.len() == 8);
        assert!(moves.contains(&Move::new_friendly(Square::b7, Square::a8, Piece::WhiteKing, Piece::None, false, false, false, false)));
        assert!(moves.contains(&Move::new_friendly(Square::b7, Square::b6, Piece::WhiteKing, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn king_cannot_move_over_edge() {
        let mut game = Game::new_from_fen("8/K7/8/4k3/8/8/8/8 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a7);
        assert!(moves.len() == 5);
        assert!(moves.contains(&Move::new_friendly(Square::a7, Square::a8, Piece::WhiteKing, Piece::None, false, false, false, false)));
        assert!(moves.contains(&Move::new_friendly(Square::a7, Square::b6, Piece::WhiteKing, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn rook_has_no_legal_moves_at_start() {
        let mut game = Game::new_from_start_pos();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a1);
        assert!(moves.len() == 0);
    }

    #[test]
    pub fn  queen_has_no_legal_moves_at_start() {
        let mut game = Game::new_from_start_pos();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::d1);
        assert!(moves.len() == 0);
    }

    #[test]
    pub fn queen_has_correct_number_of_legal_moves_on_open_board() {
        let mut game = Game::new_from_fen("K7/8/8/8/3Q4/8/8/7k w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::d4);
        assert!(moves.len() == 27);
        assert!(moves.contains(&Move::new_friendly(Square::d4, Square::d1, Piece::WhiteQueen, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn rook_has_correct_number_of_legal_moves_on_open_board() {
        let mut game = Game::new_from_fen("K7/8/8/8/3R4/8/8/7k w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::d4);
        assert!(moves.len() == 14);
    }

    #[test]
    pub fn bishop_has_correct_number_of_legal_moves_on_open_board() {
        let mut game = Game::new_from_fen("K7/8/8/8/3B4/8/8/7k w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::d4);
        assert!(moves.len() == 13);
    }

    #[test]
    pub fn queen_has_correct_number_of_moves_when_friendlies_in_the_way() {
        let mut game = Game::new_from_fen("8/1KR5/1QN5/1BB5/8/8/8/7k w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::b6);
        assert!(moves.len() == 3);
    }

    #[test]
    pub fn queen_has_correct_number_of_moves_when_enemies_in_the_way() {
        let mut game = Game::new_from_fen("7K/1rr5/1Qb5/1nb5/8/8/8/7k w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::b6);
        assert!(moves.len() == 8);
    }

    #[test]
    pub fn castling_moves_are_found_for_white() {
        let mut game = Game::new_from_fen("4k3/8/8/8/8/8/8/R3K2R w KQ - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e1);
        assert!(moves.contains(&Move::new_friendly(Square::e1, Square::g1, Piece::WhiteKing, Piece::None, false, false, false, true)));
        assert!(moves.contains(&Move::new_friendly(Square::e1, Square::c1, Piece::WhiteKing, Piece::None, false, false, false, true)));
    }

    #[test]
    pub fn castling_moves_are_found_for_black() {
        let mut game = Game::new_from_fen("r3k2r/8/8/8/8/8/8/4K3 b kq - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e8);
        assert!(moves.contains(&Move::new_friendly(Square::e8, Square::g8, Piece::BlackKing, Piece::None, false, false, false, true)));
        assert!(moves.contains(&Move::new_friendly(Square::e8, Square::c8, Piece::BlackKing, Piece::None, false, false, false, true)));
    }

    #[test]
    pub fn castling_moves_are_not_found_when_unavailable() {
        let mut game = Game::new_from_fen("4k3/8/8/8/8/8/8/R3K2R w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e1);
        assert!(moves.len() == 5);
    }

    #[test]
    pub fn cant_castle_if_pieces_in_the_way() {
        let mut game = Game::new_from_fen("4k3/8/8/8/8/8/8/RR2K1NR w KQ - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e1);
        assert_eq!(moves.len(), 5);
    }

    #[test]
    pub fn cant_move_king_into_rook_line_of_attack() {
        let mut game = Game::new_from_fen("kr6/8/8/8/8/8/8/K7 w - - 0 25").unwrap();
        generate_moves(&mut game, MoveTypes::All).print();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a1);
        assert_eq!(moves.iter().filter(|m| is_legal(&mut game, m)).count(), 1);
    }

    #[test]
    pub fn bishop_has_correct_number_of_legal_moves() {
        let mut game = Game::new_from_fen("K6k/B7/8/8/8/8/8/8 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a7);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    pub fn cant_move_blocking_piece_if_king_is_pinned() {
        let mut game = Game::new_from_fen("K6k/B7/r7/8/8/8/8/8 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a7);
        assert_eq!(moves.iter().any(|m| is_legal(&mut game, m)), false);
    }

    #[test]
    pub fn cant_castle_if_in_check() {
        let mut game = Game::new_from_fen("k7/8/8/4r3/8/8/8/4K2R w K - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e1);
        assert_eq!(moves.into_iter().any(|m| m.is_castling()), false);
    }

    #[test]
    pub fn is_in_check_is_true_when_in_check_by_rook() {
        let game = Game::new_from_fen("k7/8/8/8/4r3/8/8/4K3 w K - 0 25").unwrap();
        assert_eq!(game.is_in_check(Color::White), true);
    }

    #[test]
    pub fn rooks_should_have_5_moves_here() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/8/RNBQKBNR w K - 0 25").unwrap();
        assert_eq!(generate_moves(&mut game, MoveTypes::All).all_from(Square::h1).len(), 6);
        assert_eq!(generate_moves(&mut game, MoveTypes::All).all_from(Square::a1).len(), 6);
    }

    #[test]
    pub fn rook_should_have_a_capture_move() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/8/RNBQKBNR w K - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a1);
        assert_eq!(moves.contains(&Move::new_friendly(Square::a1, Square::a7, Piece::WhiteRook, Piece::None, true, false, false, false)), true);
    }

    #[test]
    pub fn knight_should_have_a_capture_move() {
        let mut game = Game::new_from_fen("rnbqkbnr/p1pppppp/8/8/8/1p6/8/N1BQKBNR w K - 0 25").unwrap();
        assert_eq!(generate_moves(&mut game, MoveTypes::All).all_from(Square::a1).contains(&Move::new_friendly(Square::a1, Square::b3, Piece::WhiteKnight, Piece::None, true, false, false, false)), true);
    }

    #[test]
    pub fn bishop_should_have_a_capture_move() {
        let mut game = Game::new_from_fen("rnbqkbnr/p1pppppp/8/1p6/4P3/8/PPPP1PPP/RNBQKBNR w K - 0 25").unwrap();
        assert_eq!(generate_moves(&mut game, MoveTypes::All).all_from(Square::f1).contains(&Move::new_friendly(Square::f1, Square::b5, Piece::WhiteBishop, Piece::None, true, false, false, false)), true);
    }

    #[test]
    pub fn rook_captured_by_pawn_generates_right_move() {
        let mut game = Game::new_from_fen("1nbqkbnr/1ppppppp/8/8/r7/1P6/P1PPPPPP/RNBQKBNR w K - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::b3);
        assert_eq!(moves.contains(&Move::new_friendly(Square::b3, Square::a4, Piece::WhitePawn, Piece::None, true, false, false, false)), true);
    }

    #[test]
    pub fn pawns_cant_capture_straight() {
        let mut game = Game::new_from_fen("k7/8/8/p7/P7/8/8/K7 w K - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::d8);
        assert_eq!(moves.contains(&Move::new_friendly(Square::a4, Square::a5, Piece::WhitePawn, Piece::None, true, false, false, false)), false);
    }

    #[test]
    pub fn pawns_cant_move_straight_into_piece() {
        let mut game = Game::new_from_fen("k7/8/8/p7/P7/8/8/K7 w K - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a4);
        assert_eq!(moves.contains(&Move::new_friendly(Square::a4, Square::a5, Piece::WhitePawn, Piece::None, false, false, false, false)), false);
    }

    #[test]
    pub fn rook_should_not_have_illegal_moves() {
        let mut game = Game::new_from_fen("r1bqkbnr/pppppppp/2n5/1P6/8/8/2PPPPPP/RNBQKBNR b KQkq - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a8);
        assert_eq!(moves.len(), 1);
    }

    #[test]
    pub fn cant_castle_if_path_is_under_attack() {
        let mut game = Game::new_from_fen("rnbqkbn1/ppppppp1/8/8/8/4BNP1/PPPPPrP1/RNBQK2R w KQq - 0 8").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e1);
        assert_eq!(moves.contains(&Move::new_friendly(Square::e1, Square::g1, Piece::WhiteKing, Piece::None, false, false, false, true)), false);
    }

    #[test]
    pub fn can_castle_when_its_open() {
        let mut game = Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 10").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e1);
        assert_eq!(moves.contains(&Move::new_friendly(Square::e1, Square::g1, Piece::WhiteKing, Piece::None, false, false, false, true)), true);
    }

    #[test]
    pub fn cant_castle_when_a_paw_is_in_front_of_king() {
        let mut game = Game::new_from_fen("r3k2r/4P3/8/8/8/8/8/4K3 b kq - 0 10").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e8);
        assert_eq!(moves.contains(&Move::new_friendly(Square::e8, Square::g8, Piece::WhiteKing, Piece::None, false, false, false, true)), false);
        assert_eq!(moves.contains(&Move::new_friendly(Square::e8, Square::c8, Piece::WhiteKing, Piece::None, false, false, false, true)), false);
    }
}
