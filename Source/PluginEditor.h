#pragma once
#include "PluginProcessor.h"
#include <juce_gui_basics/juce_gui_basics.h>
#include <juce_graphics/juce_graphics.h>

class GradientKnob : public juce::Component, private juce::Timer
{
public:
    GradientKnob(juce::AudioProcessorValueTreeState& s);
    void paint(juce::Graphics& g) override;
    void mouseDown(const juce::MouseEvent& e) override;
    void mouseDrag(const juce::MouseEvent& e) override;
private:
    void timerCallback() override;
    void updateFromMouse(juce::Point<float> p);
    static juce::Colour lerpColor(float t);
    static juce::Point<float> angleToPos(juce::Point<float> c, float r, float aRad);
    void roundedLine(juce::Graphics& g, juce::Point<float> a, juce::Point<float> b, float w, juce::Colour col);

    juce::AudioProcessorValueTreeState& apvts;
    juce::AudioParameterFloat* param = nullptr;
    float target = 50.0f, current = 50.0f;

    static constexpr float START_DEG = 120.0f;
    static constexpr float SWEEP_DEG = 300.0f;
    static constexpr float DEAD_START = 60.0f;
    static constexpr float DEAD_END = 120.0f;
};

class KnobMuseAudioProcessorEditor : public juce::AudioProcessorEditor
{
public:
    KnobMuseAudioProcessorEditor(KnobMuseAudioProcessor&);
private:
    KnobMuseAudioProcessor& proc;
    GradientKnob knob;
};