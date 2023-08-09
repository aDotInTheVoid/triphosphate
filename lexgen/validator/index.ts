import { ValidationResult } from '@atproto/lexicon';
import { lexicons } from './lexicon/lexicons';

export function triphosphate_bridge_validate(type: string, input: any): ValidationResult {
    return lexicons.validate(type, input);
}
