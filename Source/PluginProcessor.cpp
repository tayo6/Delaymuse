#include "PluginProcessor.h"
#include "PluginEditor.h"

KnobMuseAudioProcessor::KnobMuseAudioProcessor()
: AudioProcessor(BusesProperties()),
  apvts(*this, nullptr, "Params", createParams()) {}

KnobMuseAudioProcessor::~KnobMuseAudioProcessor() {}

juce::AudioProcessorValueTreeState::ParameterLayout KnobMuseAudioProcessor::createParams()
{
    std::vector<std::unique_ptr<juce::RangedAudioParameter>> p;
    p.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID{"knob", 1}, "Knob",
        juce::NormalisableRange<float>(0.0f, 100.0f), 50.0f,
        juce::AudioParameterFloatAttributes().withStringFromValueFunction([](float v, int){ return juce::String((int)v) + "%"; })
    ));
    return { p.begin(), p.end() };
}

juce::AudioProcessorEditor* KnobMuseAudioProcessor::createEditor()
{ return new KnobMuseAudioProcessorEditor(*this); }