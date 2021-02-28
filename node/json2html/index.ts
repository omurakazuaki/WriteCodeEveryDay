import { json2html } from './libs/json2html';

console.log(json2html(`[
    {
        "code": 1,
        "name": "北海道"
    },
    {
        "code": 2,
        "name": "青森県"
    }
]`));
