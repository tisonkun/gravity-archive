/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2022-present tison <wander4096@gmail.com>
 */

package org.tisonkun.gravity.model.github.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Builder;
import lombok.Data;
import lombok.extern.jackson.Jacksonized;

@Jacksonized
@Builder
@Data
public class License {
    private final String key;
    private final String name;
    @JsonProperty("spdx_id")
    private final String spdxId;
    @JsonProperty("node_id")
    private final String nodeId;
    private final String url;
}
