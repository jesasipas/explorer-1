"""Tests de la logique de téléopération (sans ROS ni matériel)."""

from __future__ import annotations

import pytest

from explorer_operator.teleop import (
    STOP,
    TeleopLimits,
    apply_deadzone,
    axes_to_velocity,
)

LIMITS = TeleopLimits(max_linear=0.5, max_angular=2.0, deadzone=0.1)


@pytest.mark.parametrize("value", [0.0, 0.05, -0.1, 0.1])
def test_deadzone_cancels_small_inputs(value: float) -> None:
    assert apply_deadzone(value, 0.1) == 0.0


def test_deadzone_is_continuous_and_reaches_full_scale() -> None:
    assert apply_deadzone(1.0, 0.1) == pytest.approx(1.0)
    assert apply_deadzone(-1.0, 0.1) == pytest.approx(-1.0)
    # Juste après la zone morte, la sortie part de ~0 (pas de saut).
    assert apply_deadzone(0.11, 0.1) == pytest.approx(0.0111, abs=1e-3)


def test_full_stick_gives_max_speed() -> None:
    velocity = axes_to_velocity(1.0, -1.0, 1.0, limits=LIMITS)
    assert velocity.vx == pytest.approx(0.5)
    assert velocity.vy == pytest.approx(-0.5)
    assert velocity.wz == pytest.approx(2.0)


def test_out_of_range_inputs_are_clamped() -> None:
    velocity = axes_to_velocity(3.0, 0.0, -7.0, limits=LIMITS)
    assert velocity.vx == pytest.approx(0.5)
    assert velocity.wz == pytest.approx(-2.0)


def test_speed_scale_reduces_all_axes() -> None:
    velocity = axes_to_velocity(1.0, 1.0, 1.0, speed_scale=0.5, limits=LIMITS)
    assert (velocity.vx, velocity.vy, velocity.wz) == pytest.approx((0.25, 0.25, 1.0))


def test_emergency_stop_overrides_everything() -> None:
    velocity = axes_to_velocity(1.0, 1.0, 1.0, emergency_stop=True, limits=LIMITS)
    assert velocity == STOP
    assert velocity.is_zero


def test_idle_sticks_give_exact_zero() -> None:
    assert axes_to_velocity(0.02, -0.03, 0.0, limits=LIMITS).is_zero


@pytest.mark.parametrize(
    "kwargs",
    [{"max_linear": 0.0}, {"max_angular": -1.0}, {"deadzone": 1.0}, {"deadzone": -0.1}],
)
def test_invalid_limits_are_rejected(kwargs: dict[str, float]) -> None:
    with pytest.raises(ValueError):
        TeleopLimits(**kwargs)
