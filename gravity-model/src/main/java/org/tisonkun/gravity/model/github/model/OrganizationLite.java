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
public class OrganizationLite {
    private final int id;
    private final String login;
    @JsonProperty("gravatar_id")
    private final String gravatarId;
    @JsonProperty("avatar_url")
    private final String avatarUrl;
    private final String url;
}
