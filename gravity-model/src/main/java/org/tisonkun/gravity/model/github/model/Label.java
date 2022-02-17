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
public class Label {
    private final long id;
    @JsonProperty("node_id")
    private final String nodeId;
    private final String url;
    @JsonProperty("html_url")
    private final String htmlUrl;
    private final String description;
    private final String name;
    private final String color;
    @JsonProperty("default")
    private final boolean isDefault;
}
