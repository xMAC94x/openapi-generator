/*
 * ping some object
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


package org.openapitools.client.api;

import org.openapitools.client.ApiException;
import org.openapitools.client.model.SomeObj;
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.io.InputStream;

/**
 * API tests for PingApi
 */
@Disabled
public class PingApiTest {

    private final PingApi api = new PingApi();

    /**
     * @throws ApiException if the Api call fails
     */
    @Test
    public void getPingTest() throws ApiException {
        Long petId = null;
        String name = null;
        String status = null;
        InputStream response = api.getPing(petId)
                .name(name)
                .status(status)
                .execute();
        // TODO: test validations
    }

    /**
     * @throws ApiException if the Api call fails
     */
    @Test
    public void postPingTest() throws ApiException {
        SomeObj someObj = null;
        InputStream response = api.postPing(someObj);
        // TODO: test validations
    }

}
