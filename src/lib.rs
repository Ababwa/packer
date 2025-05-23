use std::cmp::{Ordering, Reverse};
use glam::{I16Vec2, I64Vec2, I8Vec2, IVec2, U16Vec2, U64Vec2, U8Vec2, UVec2};
use glam_traits::{BVec, IntVec2};
use itertools::Itertools;
use num_traits::{AsPrimitive, Zero};

fn insert<V>(spots: &mut Vec<V>, mut new_spot: V, other_dim: usize) where V: IntVec2, V::Scalar: Ord {
	let mut min_other_d = new_spot[other_dim];
	let mut index = spots.len();
	while index != 0 {
		index -= 1;
		if spots[index].element_sum() >= new_spot.element_sum() {
			break;
		}
		if spots[index].cmple(new_spot).all() {
			min_other_d = spots.remove(index)[other_dim].min(min_other_d);
		}
	}
	new_spot[other_dim] = min_other_d;
	while index < spots.len() {
		match spots[index].element_sum().cmp(&new_spot.element_sum()) {
			Ordering::Less => break,
			Ordering::Equal => if spots[index].element_product() >= new_spot.element_product() { break },
			Ordering::Greater => {},
		}
		index += 1
	}
	spots.insert(index, new_spot);
}

pub fn pack<V, I>(rects: I) -> (Vec<V>, V)
where
	V: IntVec2,
	V::Scalar: Ord + Zero,
	I: IntoIterator<Item = V>,
	usize: AsPrimitive<V::Scalar>,
{
	let mut rects = rects.into_iter().enumerate().collect::<Vec<_>>();
	rects.sort_by_key(|(.., r)| Reverse(r.element_sum()));
	let len1 = rects.len() / 2 + rects.len() % 2;
	let len2 = rects.len() - len1;
	let mut corners = [len1, len2].map(|len| (vec![V::ZERO], Vec::<(usize, V)>::with_capacity(len), V::Scalar::zero()));
	let mut corner_index = 0;
	for (index, size) in rects {
		let (spots, packed, corner_size) = &mut corners[corner_index];
		let spot = unsafe { spots.pop().unwrap_unchecked() };//always at least 1 item at this point
		packed.push((index, spot + corner_index.as_() * size));
		insert(spots, spot + V::X * size, 1);
		insert(spots, spot + V::Y * size, 0);
		*corner_size = (spot + size).element_sum().max(*corner_size);
		corner_index ^= 1;
	}
	let [(.., pos1, size1), (.., pos2, size2)] = corners;
	let size = V::splat(size1.max(size2));
	let mut pos = pos1.into_iter().interleave(pos2.into_iter().map(|(index, pos)| (index, size - pos))).collect::<Vec<_>>();
	pos.sort_by_key(|&(index, ..)| index);
	(pos.into_iter().map(|(.., pos)| pos).collect(), size)
}

macro_rules! pack_decl {
	($name:ident, $type:ty, $glam_type:ty) => {
		pub fn $name<I>(rects: I) -> (Vec<[$type; 2]>, [$type; 2]) where I: IntoIterator<Item = [$type; 2]> {
			let (coords, size) = pack(rects.into_iter().map(|r| <$glam_type>::from(r)));
			(coords.into_iter().map(|c| c.into()).collect(), size.into())
		}
	};
}

pack_decl!(pack_i8, i8, I8Vec2);
pack_decl!(pack_u8, u8, U8Vec2);
pack_decl!(pack_i16, i16, I16Vec2);
pack_decl!(pack_u16, u16, U16Vec2);
pack_decl!(pack_i32, i32, IVec2);
pack_decl!(pack_u32, u32, UVec2);
pack_decl!(pack_i64, i64, I64Vec2);
pack_decl!(pack_u64, u64, U64Vec2);
