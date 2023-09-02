import { ValidationResult } from '@atproto/lexicon';
import { lexicons } from './lexicon/lexicons';
import { jsonToIpld } from '@atproto/common-web';

export default function triphosphate_bridge_validate(type: string, input: any): ValidationResult {
    const input_fixed = jsonToIpld(input); // converts $link
    return lexicons.validate(type, input_fixed);
}
