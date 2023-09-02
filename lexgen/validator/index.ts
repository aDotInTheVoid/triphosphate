import { ValidationError, ValidationResult } from '@atproto/lexicon';
import { jsonToIpld, ipldToJson } from '@atproto/common-web';
import * as cbor from '@ipld/dag-cbor'
import * as ui8 from 'uint8arrays'
import isEqual from 'lodash.isequal';

import { lexicons } from './lexicon/lexicons';


function array_equal(x: Uint8Array, y: Uint8Array): boolean {
    if (x.length != y.length) {
        return false;
    }

    for (let i = 0; i < x.length; i++) {
        if (x[i] != y[i]) {
            return false;
        }
    }

    return true;
}

function base64(x: Uint8Array): string {
    return ui8.toString(x, 'base64');
}

export default function triphosphate_bridge_validate(type: string, input: any, input_nums: number[]): ValidationResult {
    const input_fixed = jsonToIpld(input); // converts $link
    const input_bytes = new Uint8Array(input_nums);

    const encode_in_js_bytes: Uint8Array = cbor.encode(input_fixed);

    if (!array_equal(input_bytes, encode_in_js_bytes)) {
        return {
            success: false,
            error: new ValidationError(`CBOR arrays not equal: ${base64(input_bytes)} != ${base64(encode_in_js_bytes)}`)
        }
    }

    const decode_in_js = ipldToJson(cbor.decode(input_bytes));

    if (!isEqual(decode_in_js, input)) {
        const input_json = JSON.stringify(input);
        const roundtrip_json = JSON.stringify(decode_in_js);

        return {
            success: false,
            error: new ValidationError(`JSON not equal: ${input_json} != ${roundtrip_json}`)
        }
    }

    return lexicons.validate(type, input_fixed);
}

