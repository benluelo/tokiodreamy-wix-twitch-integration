// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CustomTextField } from './CustomTextField';
import type { OrderLineItemOption } from './OrderLineItemOption';
import type { OrderMediaItem } from './OrderMediaItem';

export interface OrderLineItem {
	index: bigint | null;
	quantity: bigint;
	name: string;
	options: Array<OrderLineItemOption>;
	customTextFields: Array<CustomTextField> | null;
	mediaItem: OrderMediaItem;
	notes: string | null;
}
