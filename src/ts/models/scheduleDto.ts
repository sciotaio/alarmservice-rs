/**
 * Alarmservice Demo
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */



export class ScheduleDto {
    'begin'?: number;
    'end'?: number;
    'days_of_week_mask'?: number;
    'roomId'?: number;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "begin",
            "baseName": "begin",
            "type": "number"
        },
        {
            "name": "end",
            "baseName": "end",
            "type": "number"
        },
        {
            "name": "days_of_week_mask",
            "baseName": "days_of_week_mask",
            "type": "number"
        },
        {
            "name": "roomId",
            "baseName": "roomId",
            "type": "number"
        }    ];

    static getAttributeTypeMap() {
        return ScheduleDto.attributeTypeMap;
    }
}
