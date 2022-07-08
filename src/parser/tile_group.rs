use nom::{
    bits::{bits, complete as bit_parsers},
    IResult,
};

use super::util::take_bool_bit;

pub fn parse_tile_group_obu<'a, 'b>(
    input: &'a [u8],
    size: usize,
    seen_frame_header: &'b mut bool,
    tile_cols: usize,
    tile_rows: usize,
    tile_cols_log2: usize,
    tile_rows_log2: usize,
) -> IResult<&'a [u8], ()> {
    // Tile group header
    let (input, (tg_start, tg_end)) = bits(|input| {
        let num_tiles = tile_cols * tile_rows;
        let (input, tile_start_and_end_present) = if num_tiles > 1 {
            take_bool_bit(input)?
        } else {
            (input, false)
        };
        if num_tiles == 1 || !tile_start_and_end_present {
            Ok((input, (0, num_tiles - 1)))
        } else {
            let tile_bits = tile_cols_log2 + tile_rows_log2;
            let (input, tg_start): (_, usize) = bit_parsers::take(tile_bits)(input)?;
            let (input, tg_end): (_, usize) = bit_parsers::take(tile_bits)(input)?;
            Ok((input, (tg_start, tg_end)))
        }
    })(input)?;

    // The actual tile group
    let (input, _) = bits(|input| {
        for tile_num in tg_start..=tg_end {
            let tile_row = tile_num / tile_cols;
            let tile_col = tile_num % tile_cols;
            todo!();
        }
        if tg_end == num_tiles - 1 {
            // These functions are irrelevant to us
            // if !disable_frame_end_update_cdf {
            //     frame_end_update_cdf()?;
            // }
            // decode_frame_wrap()?;
            *seen_frame_header = false;
        }
        Ok((input, ()))
    })(input)?;

    Ok((input, ()))
}
